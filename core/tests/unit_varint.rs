use json_packer_core::test_expose::{BitWriter, BitReader, write_uleb128, read_uleb128, write_sleb128, read_sleb128};

#[test]
fn uleb128_roundtrip() {
    let mut w = BitWriter::new();
    for &v in &[0u64, 1, 127, 128, 16384, u64::MAX] {
        write_uleb128(&mut w, v);
    }
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    for &v in &[0u64, 1, 127, 128, 16384, u64::MAX] {
        let x = read_uleb128(&mut r).unwrap();
        assert_eq!(x, v);
    }
}

#[test]
fn sleb128_roundtrip() {
    let mut w = BitWriter::new();
    for &v in &[0i64, -1, 1, 63, 64, -64, i64::MIN, i64::MAX] {
        write_sleb128(&mut w, v);
    }
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    for &v in &[0i64, -1, 1, 63, 64, -64, i64::MIN, i64::MAX] {
        let x = read_sleb128(&mut r).unwrap();
        assert_eq!(x, v);
    }
}
