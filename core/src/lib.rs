mod error;
mod base64util;
mod types;
mod bitstream;
mod varint;
mod header;

pub use error::Error;

/// 将压缩后的字节数组编码为 Base64 字符串
pub fn encode_base64(bytes: &[u8]) -> String {
    base64util::encode_base64(bytes)
}

/// 将 Base64 字符串解码为字节数组
pub fn decode_base64(s: &str) -> Result<Vec<u8>, Error> {
    base64util::decode_base64(s)
}

/// 压缩 JSON 到字节数组（占位实现：未实现）
pub fn compress_to_bytes(_value: &serde_json::Value) -> Result<Vec<u8>, Error> {
    Err(Error::Unimplemented("compress_to_bytes"))
}

/// 压缩 JSON 到 Base64 字符串（占位实现）
pub fn compress_to_base64(value: &serde_json::Value) -> Result<String, Error> {
    let bytes = compress_to_bytes(value)?;
    Ok(encode_base64(&bytes))
}

/// 从字节数组解压为 JSON（占位实现：未实现）
pub fn decompress_from_bytes(_bytes: &[u8]) -> Result<serde_json::Value, Error> {
    Err(Error::Unimplemented("decompress_from_bytes"))
}

/// 从 Base64 字符串解压为 JSON（占位实现）
pub fn decompress_from_base64(s: &str) -> Result<serde_json::Value, Error> {
    let bytes = decode_base64(s)?;
    decompress_from_bytes(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_roundtrip() {
        let data = vec![0, 1, 2, 3, 254, 255];
        let b64 = encode_base64(&data);
        let back = decode_base64(&b64).unwrap();
        assert_eq!(data, back);
    }
}
