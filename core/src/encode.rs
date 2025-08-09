use serde_json::Value;
use crate::{bitstream::BitWriter, header, dict, huffman::HuffmanCodec, types::tag, varint, Error};

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

pub fn encode_json(value: &Value, writer: &mut BitWriter, huffman: &HuffmanCodec) -> Result<(), Error> {
    encode_value(value, writer, huffman)
}

pub fn compress_to_bytes(value: &Value) -> Result<Vec<u8>, Error> {
    // 1) 统计键频
    let freq = dict::collect_keys(value);
    // 2) 构建 canonical Huffman
    let codec = HuffmanCodec::from_frequencies(&freq)?;

    // 3) 写包头 + 字典表 + 数据
    let mut writer = BitWriter::new();
    header::write_header(&mut writer, freq.len() as u64, 0);
    dict::write_dictionary(&mut writer, &freq);
    encode_json(value, &mut writer, &codec)?;

    Ok(writer.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bitstream::BitReader, header::read_header, dict::read_dictionary};
    use serde_json::json;

    #[test]
    fn encode_header_and_dict_roundtrip() {
        let value = json!({
            "name": "Alice",
            "age": 30,
            "info": {"name": "A"}
        });
        let bytes = compress_to_bytes(&value).unwrap();
        let mut reader = BitReader::new(&bytes);
        let hdr = read_header(&mut reader).unwrap();
        assert_eq!(hdr.pool_len, 0);
        let dict = read_dictionary(&mut reader).unwrap();
        // name:2, age:1, info:1
        assert_eq!(dict.get("name"), Some(&2));
        assert_eq!(dict.get("age"), Some(&1));
        assert_eq!(dict.get("info"), Some(&1));
        assert_eq!(hdr.dict_len as usize, dict.len());
    }
}
