use std::collections::HashMap;

use serde_json::Value;

use crate::{bitstream::BitWriter, varint, types::tag};

#[derive(Debug, Clone)]
pub struct StringPool {
    // id -> value
    pub entries: Vec<String>,
    // value -> id
    pub index: HashMap<String, u64>,
}

#[derive(Debug, Clone, Copy)]
pub struct PoolConfig {
    pub min_repeats: u32,
    pub min_string_len: usize,
}

impl Default for PoolConfig {
    fn default() -> Self { Self { min_repeats: 3, min_string_len: 8 } }
}

pub fn collect_string_pool(root: &Value, cfg: PoolConfig) -> StringPool {
    let mut counter: HashMap<String, u32> = HashMap::new();
    fn walk(v: &Value, counter: &mut HashMap<String, u32>) {
        match v {
            Value::String(s) => {
                *counter.entry(s.clone()).or_insert(0) += 1;
            }
            Value::Array(a) => for x in a { walk(x, counter); },
            Value::Object(m) => for (_k, x) in m { walk(x, counter); },
            _ => {}
        }
    }
    walk(root, &mut counter);

    // 过滤并排序：频次降序，其次字节序升序，确保确定性
    let mut candidates: Vec<(String, u32)> = counter
        .into_iter()
        .filter(|(s, c)| *c >= cfg.min_repeats && s.len() >= cfg.min_string_len)
        .collect();
    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let mut entries: Vec<String> = Vec::with_capacity(candidates.len());
    let mut index: HashMap<String, u64> = HashMap::with_capacity(candidates.len());
    for (i, (s, _)) in candidates.into_iter().enumerate() {
        index.insert(s.clone(), i as u64);
        entries.push(s);
    }
    StringPool { entries, index }
}

pub fn write_string_pool(writer: &mut BitWriter, pool: &StringPool) {
    for s in &pool.entries {
        // 在池里写入原始值：tag::STRING + len + bytes（不写 is_pool_ref）
        writer.write_bits(tag::STRING as u64, 3);
        let bytes = s.as_bytes();
        varint::write_uleb128(writer, bytes.len() as u64);
        for &b in bytes { writer.write_byte(b); }
    }
}


