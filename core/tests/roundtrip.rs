use json_packer_core::{compress_to_bytes, decompress_from_bytes};
use serde_json::json;

#[test]
fn roundtrip_integers() {
    let v = json!({
        "i_min": i64::MIN,
        "i_max": i64::MAX,
        "u_max": 18446744073709551615u64
    });
    let bytes = compress_to_bytes(&v).unwrap();
    let out = decompress_from_bytes(&bytes).unwrap();
    assert_eq!(v, out);
}

#[test]
fn roundtrip_unicode_and_nested() {
    let v = json!({
        "用户": {"姓名": "张三🙂", "年龄": 25},
        "tags": ["a", "b", "c"],
        "nums": [1, -2, 3, 4, 18446744073709551615u64],
        "pi": 3.141592653589793
    });
    let bytes = compress_to_bytes(&v).unwrap();
    let out = decompress_from_bytes(&bytes).unwrap();
    assert_eq!(v, out);
}