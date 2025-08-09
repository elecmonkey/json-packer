use std::collections::HashMap;
use serde_json::Value;
use crate::{bitstream::{BitWriter, BitReader}, varint, Error};

/// 收集 JSON 中所有对象键的频率统计
pub fn collect_keys(json: &Value) -> HashMap<String, u64> {
    let mut freq_map = HashMap::new();
    collect_keys_recursive(json, &mut freq_map);
    freq_map
}

/// 递归遍历 JSON 值，统计对象键的频率
fn collect_keys_recursive(value: &Value, freq_map: &mut HashMap<String, u64>) {
    match value {
        Value::Object(map) => {
            // 统计当前对象的所有键
            for (key, val) in map {
                *freq_map.entry(key.clone()).or_insert(0) += 1;
                // 递归处理值
                collect_keys_recursive(val, freq_map);
            }
        },
        Value::Array(arr) => {
            // 递归处理数组中的每个元素
            for item in arr {
                collect_keys_recursive(item, freq_map);
            }
        },
        _ => {
            // 其他类型（null, bool, number, string）无需处理
        }
    }
}

/// 写入字典表到位流
/// 格式：[KEY_COUNT(uleb128)] + 对每个键: [KEY_LEN(uleb128)][KEY_UTF8...][FREQ(uleb128)]
pub fn write_dictionary(writer: &mut BitWriter, freq_map: &HashMap<String, u64>) {
    // 写入键的总数
    varint::write_uleb128(writer, freq_map.len() as u64);
    
    // 按字典序排序键名，确保确定性输出
    let mut sorted_keys: Vec<_> = freq_map.iter().collect();
    sorted_keys.sort_by(|a, b| a.0.cmp(b.0));
    
    // 写入每个键的信息
    for (key, &freq) in sorted_keys {
        let key_bytes = key.as_bytes();
        
        // 键长度 (ULEB128)
        varint::write_uleb128(writer, key_bytes.len() as u64);
        
        // 键内容 (UTF-8 字节)
        for &byte in key_bytes {
            writer.write_byte(byte);
        }
        
        // 键频率 (ULEB128)
        varint::write_uleb128(writer, freq);
    }
}

/// 从位流读取字典表
/// 返回键频率映射表
pub fn read_dictionary(reader: &mut BitReader) -> Result<HashMap<String, u64>, Error> {
    let mut freq_map = HashMap::new();
    
    // 读取键的总数
    let key_count = varint::read_uleb128(reader)?;
    
    for _ in 0..key_count {
        // 读取键长度
        let key_len = varint::read_uleb128(reader)? as usize;
        
        // 读取键内容
        let mut key_bytes = Vec::with_capacity(key_len);
        for _ in 0..key_len {
            key_bytes.push(reader.read_byte()?);
        }
        
        // 转换为 UTF-8 字符串
        let key = String::from_utf8(key_bytes)?;
        
        // 读取频率
        let freq = varint::read_uleb128(reader)?;
        
        freq_map.insert(key, freq);
    }
    
    Ok(freq_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_collect_keys_simple() {
        let json = json!({
            "name": "Alice",
            "age": 30
        });
        
        let freq = collect_keys(&json);
        assert_eq!(freq.len(), 2);
        assert_eq!(freq.get("name"), Some(&1));
        assert_eq!(freq.get("age"), Some(&1));
    }

    #[test]
    fn test_collect_keys_nested() {
        let json = json!({
            "user": {
                "name": "Bob",
                "profile": {
                    "name": "Bob Smith"
                }
            },
            "settings": {
                "theme": "dark"
            }
        });
        
        let freq = collect_keys(&json);
        assert_eq!(freq.get("name"), Some(&2)); // 出现2次
        assert_eq!(freq.get("user"), Some(&1));
        assert_eq!(freq.get("profile"), Some(&1));
        assert_eq!(freq.get("settings"), Some(&1));
        assert_eq!(freq.get("theme"), Some(&1));
    }

    #[test]
    fn test_collect_keys_with_arrays() {
        let json = json!({
            "items": [
                {"name": "item1", "value": 10},
                {"name": "item2", "value": 20}
            ]
        });
        
        let freq = collect_keys(&json);
        assert_eq!(freq.get("name"), Some(&2)); // 在数组元素中出现2次
        assert_eq!(freq.get("value"), Some(&2)); // 在数组元素中出现2次
        assert_eq!(freq.get("items"), Some(&1)); // 根级别出现1次
    }

    #[test]
    fn test_dictionary_roundtrip() {
        let mut freq_map = HashMap::new();
        freq_map.insert("alpha".to_string(), 10);
        freq_map.insert("beta".to_string(), 5);
        freq_map.insert("gamma".to_string(), 15);
        
        // 写入字典
        let mut writer = crate::bitstream::BitWriter::new();
        write_dictionary(&mut writer, &freq_map);
        let bytes = writer.into_bytes();
        
        // 读取字典
        let mut reader = crate::bitstream::BitReader::new(&bytes);
        let restored_freq = read_dictionary(&mut reader).unwrap();
        
        assert_eq!(freq_map, restored_freq);
    }

    #[test]
    fn test_empty_dictionary() {
        let freq_map = HashMap::new();
        
        let mut writer = crate::bitstream::BitWriter::new();
        write_dictionary(&mut writer, &freq_map);
        let bytes = writer.into_bytes();
        
        let mut reader = crate::bitstream::BitReader::new(&bytes);
        let restored_freq = read_dictionary(&mut reader).unwrap();
        
        assert!(restored_freq.is_empty());
    }

    #[test]
    fn test_unicode_keys() {
        let json = json!({
            "用户": "张三",
            "年龄": 25,
            "🚀": "rocket"
        });
        
        let freq = collect_keys(&json);
        assert_eq!(freq.len(), 3);
        assert_eq!(freq.get("用户"), Some(&1));
        assert_eq!(freq.get("年龄"), Some(&1));
        assert_eq!(freq.get("🚀"), Some(&1));
        
        // 测试往返
        let mut writer = crate::bitstream::BitWriter::new();
        write_dictionary(&mut writer, &freq);
        let bytes = writer.into_bytes();
        
        let mut reader = crate::bitstream::BitReader::new(&bytes);
        let restored_freq = read_dictionary(&mut reader).unwrap();
        
        assert_eq!(freq, restored_freq);
    }
}