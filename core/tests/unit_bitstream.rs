use json_packer::test_expose::{BitReader, BitWriter};

#[test]
fn bitwriter_reader_bits() {
    let mut w = BitWriter::new();
    w.write_bits(0b101, 3);
    w.write_bits(0b11, 2);
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    assert_eq!(r.read_bits(3).unwrap(), 0b101);
    assert_eq!(r.read_bits(2).unwrap(), 0b11);
}

#[test]
fn cross_byte_read_write() {
    let mut w = BitWriter::new();
    w.write_bits(0b11010, 5);
    w.write_bits(0b10101010101, 11);
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    assert_eq!(r.read_bits(5).unwrap(), 0b11010);
    assert_eq!(r.read_bits(11).unwrap(), 0b10101010101);
}
