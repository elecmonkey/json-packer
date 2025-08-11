use json_packer::{test_expose::{BitWriter, BitReader, write_header, read_header, VERSION_V2, collect_string_pool}, PoolConfig};
use serde_json::json;

#[test]
fn collect_string_pool_basic() {
    let v = json!({
        "a": "status_ok",
        "b": ["status_ok", "status_ok", "status_err"],
        "c": {"x": "status_ok", "y": "status_err"}
    });
    let cfg = PoolConfig { min_repeats: 3, min_string_len: 5 };
    let pool = collect_string_pool(&v, cfg);
    // status_ok 重复 4 次，应入池；status_err 2 次，不入
    assert!(pool.index.contains_key("status_ok"));
    assert!(!pool.index.contains_key("status_err"));
}

#[test]
fn write_pool_and_header_v2() {
    let mut w = BitWriter::new();
    // header v2, dict_len=0, pool_len=2
    write_header(&mut w, VERSION_V2, 0, 2);
    let bytes = w.into_bytes();
    let mut r = BitReader::new(&bytes);
    let hdr = read_header(&mut r).unwrap();
    assert_eq!(hdr.version, VERSION_V2);
    assert_eq!(hdr.dict_len, 0);
    assert_eq!(hdr.pool_len, 2);
}
