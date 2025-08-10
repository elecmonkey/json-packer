use serde_json::{Map, Value};

use crate::{
    bitstream::BitReader,
    dict,
    header,
    huffman::HuffmanCodec,
    types::tag,
    varint,
    Error,
};

fn decode_value(reader: &mut BitReader, huffman: &HuffmanCodec) -> Result<Value, Error> {
    let t = reader.read_bits(3)? as u8;
    match t {
        tag::NULL => Ok(Value::Null),
        tag::BOOL_FALSE => Ok(Value::Bool(false)),
        tag::BOOL_TRUE => Ok(Value::Bool(true)),
        tag::INT => {
            let is_unsigned = reader.read_bits(1)? as u8;
            if is_unsigned == 0 {
                let i = varint::read_sleb128(reader)?;
                Ok(Value::Number(i.into()))
            } else {
                let u = varint::read_uleb128(reader)?;
                // 尝试优先 as_u64 构造 JSON Number；serde_json::Number 支持 u64
                Ok(serde_json::Number::from(u).into())
            }
        }
        tag::FLOAT => {
            let bits = reader.read_bits(64)?;
            let f = f64::from_bits(bits);
            if !f.is_finite() { return Err(Error::IllegalFloat); }
            Ok(serde_json::Number::from_f64(f).map(Value::Number).ok_or(Error::IllegalFloat)?)
        }
        tag::STRING => {
            let len = varint::read_uleb128(reader)? as usize;
            let mut bytes = Vec::with_capacity(len);
            for _ in 0..len { bytes.push(reader.read_byte()?); }
            let s = String::from_utf8(bytes)?;
            Ok(Value::String(s))
        }
        tag::ARRAY => {
            let count = varint::read_uleb128(reader)? as usize;
            let mut arr = Vec::with_capacity(count);
            for _ in 0..count { arr.push(decode_value(reader, huffman)?); }
            Ok(Value::Array(arr))
        }
        tag::OBJECT => {
            let count = varint::read_uleb128(reader)? as usize;
            let mut map = Map::with_capacity(count);
            for _ in 0..count {
                let key = if huffman.try_get_code("").is_some() && false {
                    // 不会进入，仅为避免单符号/空树误用；真实路径走 decode_key
                    String::new()
                } else {
                    huffman.decode_key(reader)?
                };
                let val = decode_value(reader, huffman)?;
                map.insert(key, val);
            }
            Ok(Value::Object(map))
        }
        _ => Err(Error::HuffmanError),
    }
}

pub fn decode_json(reader: &mut BitReader) -> Result<Value, Error> {
    // 读包头
    let _hdr = header::read_header(reader)?;
    // 读字典并构建 Huffman
    let freq = dict::read_dictionary(reader)?;
    let codec = HuffmanCodec::from_frequencies(&freq)?;
    // 读数据区
    decode_value(reader, &codec)
}

pub fn decompress_from_bytes(bytes: &[u8]) -> Result<Value, Error> {
    let mut reader = BitReader::new(bytes);
    decode_json(&mut reader)
}