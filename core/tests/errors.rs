use json_packer_core::{compress_to_bytes, CompressOptions};
use serde_json::json;

#[test]
fn bad_magic() {
    // 手工构造错误 MAGIC + 合法版本 + dict_len=0 + pool_len=0（varint 0x00）
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"BAD!");
    bytes.push(0x01);
    bytes.push(0x00); // dict_len
    bytes.push(0x00); // pool_len
    let err = json_packer_core::decompress_from_bytes(&bytes).unwrap_err();
    assert!(matches!(err, json_packer_core::Error::BadMagic));
}

#[test]
fn bad_version() {
    // 正确 magic + 错误版本 + dict_len=0 + pool_len=0（varint 0x00）
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"JCPR");
    bytes.push(0xFF);
    bytes.push(0x00); // dict_len
    bytes.push(0x00); // pool_len
    let err = json_packer_core::decompress_from_bytes(&bytes).unwrap_err();
    assert!(matches!(err, json_packer_core::Error::BadVersion));
}

#[test]
fn truncated_data() {
    let v = json!({"a": 1});
    let mut bytes = compress_to_bytes(&v, &CompressOptions::default()).unwrap();
    bytes.truncate(bytes.len().saturating_sub(3));
    let err = json_packer_core::decompress_from_bytes(&bytes).unwrap_err();
    // 可能触发 BitstreamOutOfBounds 或 VarintError
    matches!(err, json_packer_core::Error::BitstreamOutOfBounds | json_packer_core::Error::VarintError);
}

#[test]
fn utf8_error_in_string() {
    // 构造最小合法头 + 空字典 + 一个字符串，长度=1，字节=0xFF（非法 UTF-8）
    let mut bytes = Vec::new();
    // MAGIC + VERSION
    bytes.extend_from_slice(b"JCPR");
    bytes.push(0x01);
    // dict_len=0, pool_len=0
    bytes.push(0x00);
    bytes.push(0x00);
    // DATA: tag::STRING(101)
    bytes.push(0b101);
    // len=1 (uleb128)
    bytes.push(0x01);
    // invalid utf-8 byte
    bytes.push(0xFF);

    let err = json_packer_core::decompress_from_bytes(&bytes).unwrap_err();
    matches!(err, json_packer_core::Error::Utf8(_));
}
