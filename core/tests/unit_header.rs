use json_packer::test_expose::{BitWriter, BitReader, write_header, read_header, MAGIC, VERSION_V1};

#[test]
fn header_roundtrip() {
    let mut w = BitWriter::new();
    write_header(&mut w, VERSION_V1, 12, 0);
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    let h = read_header(&mut r).unwrap();
    assert_eq!(h.version, VERSION_V1);
    assert_eq!(h.dict_len, 12);
    assert_eq!(h.pool_len, 0);
}

#[test]
fn header_magic_version_constants() {
    assert_eq!(&MAGIC, b"JCPR");
    assert_eq!(VERSION_V1, 0x01);
}
