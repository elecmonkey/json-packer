use serde_json::Value;
use crate::{bitstream::BitWriter, header, dict, huffman::HuffmanCodec, types::tag, varint, Error, pool::{collect_string_pool, PoolConfig, write_string_pool}};
use once_cell::sync::Lazy;
use std::sync::RwLock;

fn encode_value(value: &Value, writer: &mut BitWriter, huffman: &HuffmanCodec) -> Result<(), Error> {
    match value {
        Value::Null => {
            writer.write_bits(tag::NULL as u64, 3);
        }
        Value::Bool(b) => {
            writer.write_bits((if *b { tag::BOOL_TRUE } else { tag::BOOL_FALSE }) as u64, 3);
        }
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                writer.write_bits(tag::INT as u64, 3);
                // is_unsigned = 0
                writer.write_bits(0, 1);
                varint::write_sleb128(writer, i);
            } else if let Some(u) = n.as_u64() {
                writer.write_bits(tag::INT as u64, 3);
                // is_unsigned = 1
                writer.write_bits(1, 1);
                varint::write_uleb128(writer, u);
            } else if let Some(f) = n.as_f64() {
                if !f.is_finite() { return Err(Error::IllegalFloat); }
                writer.write_bits(tag::FLOAT as u64, 3);
                writer.write_bits(f.to_bits(), 64);
            } else {
                return Err(Error::IllegalFloat);
            }
        }
        Value::String(s) => {
            writer.write_bits(tag::STRING as u64, 3);
            let bytes = s.as_bytes();
            varint::write_uleb128(writer, bytes.len() as u64);
            for &b in bytes { writer.write_byte(b); }
        }
        Value::Array(arr) => {
            writer.write_bits(tag::ARRAY as u64, 3);
            varint::write_uleb128(writer, arr.len() as u64);
            for item in arr { encode_value(item, writer, huffman)?; }
        }
        Value::Object(map) => {
            writer.write_bits(tag::OBJECT as u64, 3);
            varint::write_uleb128(writer, map.len() as u64);
            for (k, v) in map {
                huffman.write_key_code(k, writer)?;
                encode_value(v, writer, huffman)?;
            }
        }
    }
    Ok(())
}


pub fn compress_to_bytes(value: &Value) -> Result<Vec<u8>, Error> {
    compress_with_options(value, &GLOBAL_COMPRESS_OPTIONS.read().unwrap())
}

#[derive(Debug, Clone)]
pub struct CompressOptions {
    pub enable_value_pool: bool,
    pub pool_min_repeats: u32,
    pub pool_min_string_len: usize,
}

impl Default for CompressOptions {
    fn default() -> Self { Self { enable_value_pool: false, pool_min_repeats: 3, pool_min_string_len: 8 } }
}

pub fn compress_with_options(value: &Value, opt: &CompressOptions) -> Result<Vec<u8>, Error> {
    // 1) 统计键频
    let freq = dict::collect_keys(value);
    // 2) 构建 canonical Huffman
    let codec = HuffmanCodec::from_frequencies(&freq)?;

    // 3) 值池（仅字符串，按需）
    let (version, pool_len, string_pool) = if opt.enable_value_pool {
        let pool = collect_string_pool(value, PoolConfig { min_repeats: opt.pool_min_repeats, min_string_len: opt.pool_min_string_len });
        (header::VERSION_V2, pool.entries.len() as u64, Some(pool))
    } else {
        (header::VERSION_V1, 0, None)
    };

    // 4) 写包头 + 字典表 + 值池 + 数据
    let mut writer = BitWriter::new();
    header::write_header(&mut writer, version, freq.len() as u64, pool_len);
    dict::write_dictionary(&mut writer, &freq);
    if let Some(pool) = &string_pool {
        write_string_pool(&mut writer, pool);
    }
    // 统一分派编码（启用/禁用值池皆可）
    encode_value_dispatch(value, &mut writer, &codec, string_pool.as_ref())?;

    Ok(writer.into_bytes())
}

pub static GLOBAL_COMPRESS_OPTIONS: Lazy<RwLock<CompressOptions>> = Lazy::new(|| RwLock::new(CompressOptions::default()));

/// 设置全局压缩选项（影响后续 compress_to_* 调用）
pub fn set_global_compress_options(opts: CompressOptions) {
    if let Ok(mut guard) = GLOBAL_COMPRESS_OPTIONS.write() {
        *guard = opts;
    }
}

/// 获取当前全局压缩选项（拷贝）
pub fn get_global_compress_options() -> CompressOptions {
    GLOBAL_COMPRESS_OPTIONS.read().map(|g| g.clone()).unwrap_or_default()
}

fn encode_value_dispatch(value: &Value, writer: &mut BitWriter, huffman: &HuffmanCodec, string_pool: Option<&crate::pool::StringPool>) -> Result<(), Error> {
    match value {
        Value::String(s) => {
            if let Some(pool) = string_pool {
                if let Some(&id) = pool.index.get(s) {
                    writer.write_bits(tag::STRING as u64, 3);
                    writer.write_bits(1, 1); // is_pool_ref
                    varint::write_uleb128(writer, id);
                    return Ok(());
                }
            }
            // 非引用路径（与原逻辑相同，但需要写 is_pool_ref=0 于 v2）
            if string_pool.is_some() {
                writer.write_bits(tag::STRING as u64, 3);
                writer.write_bits(0, 1);
                let bytes = s.as_bytes();
                varint::write_uleb128(writer, bytes.len() as u64);
                for &b in bytes { writer.write_byte(b); }
                return Ok(());
            }
            // v1：保持原逻辑（无额外 is_pool_ref）
            writer.write_bits(tag::STRING as u64, 3);
            let bytes = s.as_bytes();
            varint::write_uleb128(writer, bytes.len() as u64);
            for &b in bytes { writer.write_byte(b); }
            Ok(())
        }
        Value::Array(a) => {
            writer.write_bits(tag::ARRAY as u64, 3);
            varint::write_uleb128(writer, a.len() as u64);
            for x in a { encode_value_dispatch(x, writer, huffman, string_pool)?; }
            Ok(())
        }
        Value::Object(m) => {
            writer.write_bits(tag::OBJECT as u64, 3);
            varint::write_uleb128(writer, m.len() as u64);
            for (k, v) in m {
                huffman.write_key_code(k, writer)?;
                encode_value_dispatch(v, writer, huffman, string_pool)?;
            }
            Ok(())
        }
        _ => encode_value(value, writer, huffman),
    }
}