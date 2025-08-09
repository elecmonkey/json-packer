mod error;
mod base64util;
mod types;
mod bitstream;
mod varint;
mod header;
mod dict;
mod huffman;
mod encode;
mod decode;

pub use error::Error;
pub use base64util::{encode_base64, decode_base64};
pub use dict::{collect_keys, write_dictionary, read_dictionary};
pub use huffman::HuffmanCodec;

/// 压缩 JSON 到字节数组
pub fn compress_to_bytes(value: &serde_json::Value) -> Result<Vec<u8>, Error> { encode::compress_to_bytes(value) }

/// 压缩 JSON 到 Base64 字符串
pub fn compress_to_base64(value: &serde_json::Value) -> Result<String, Error> {
    let bytes = compress_to_bytes(value)?;
    Ok(encode_base64(&bytes))
}

/// 从字节数组解压为 JSON
pub fn decompress_from_bytes(bytes: &[u8]) -> Result<serde_json::Value, Error> { decode::decompress_from_bytes(bytes) }

/// 从 Base64 字符串解压为 JSON
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
