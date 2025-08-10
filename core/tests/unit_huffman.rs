use json_packer_core::HuffmanCodec;
use std::collections::HashMap;
use json_packer_core::test_expose::{BitWriter, BitReader};

#[test]
fn huffman_build_basic() {
    let mut freq = HashMap::new();
    freq.insert("name".to_string(), 2);
    freq.insert("age".to_string(), 1);
    let codec = HuffmanCodec::from_frequencies(&freq).unwrap();
    assert!(codec.try_get_code("name").is_some());
    assert!(codec.try_get_code("age").is_some());
}

#[test]
fn huffman_encode_decode_roundtrip() {
    let mut freq = HashMap::new();
    freq.insert("name".to_string(), 2);
    freq.insert("age".to_string(), 1);
    let codec = HuffmanCodec::from_frequencies(&freq).unwrap();
    let mut w = BitWriter::new();
    codec.write_key_code("name", &mut w).unwrap();
    codec.write_key_code("age", &mut w).unwrap();
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    let k1 = codec.decode_key(&mut r).unwrap();
    let k2 = codec.decode_key(&mut r).unwrap();
    assert_eq!(k1, "name");
    assert_eq!(k2, "age");
}

#[test]
fn huffman_single_and_empty_cases() {
    // single symbol
    let mut one = HashMap::new();
    one.insert("only".to_string(), 10);
    let codec1 = HuffmanCodec::from_frequencies(&one).unwrap();
    assert!(codec1.try_get_code("only").is_some());

    // empty
    let empty: HashMap<String, u64> = HashMap::new();
    let codec2 = HuffmanCodec::from_frequencies(&empty).unwrap();
    assert!(codec2.try_get_code("x").is_none());
}
