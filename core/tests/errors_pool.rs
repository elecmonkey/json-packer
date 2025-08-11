use json_packer::test_expose::{BitWriter, write_header, VERSION_V2};
use json_packer::{decompress_from_bytes, Error};

#[test]
fn pool_missing_should_error() {
    // 构造 v2 header + dict_len=0 + pool_len=0，随后数据区引用 id=0（池缺失/越界）
    let mut w = BitWriter::new();
    write_header(&mut w, VERSION_V2, 0, 0);
    // 字典区：key_count=0
    json_packer::test_expose::write_uleb128(&mut w, 0);
    // 数据区：string 引用 id=0
    // tag=101 + is_ref=1 + id=0
    w.write_bits(0b101, 3);
    w.write_bits(1, 1);
    w.write_bits(0, 8); // uleb128(0) 一个字节 0x00
    let bytes = w.into_bytes();
    let err = decompress_from_bytes(&bytes).unwrap_err();
    assert!(matches!(err, Error::PoolIdOutOfRange));
}

#[test]
fn pool_id_out_of_range() {
    // v2 header + pool_len=0，数据区引用 id=1
    let mut w = BitWriter::new();
    write_header(&mut w, VERSION_V2, 0, 0);
    // 字典区：key_count=0
    json_packer::test_expose::write_uleb128(&mut w, 0);
    w.write_bits(0b101, 3);
    w.write_bits(1, 1);
    // uleb128(1)
    w.write_bits(1, 8); // 写一个字节 0x01
    let bytes = w.into_bytes();
    let err = decompress_from_bytes(&bytes).unwrap_err();
    assert!(matches!(err, Error::PoolIdOutOfRange));
}
