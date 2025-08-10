use serde_json::json;
use json_packer_core::{compress_to_bytes, decompress_from_bytes};
use json_packer_core::test_expose::{compress_with_options, CompressOptions};

#[test]
fn v2_string_pool_roundtrip_and_benefit() {
    let v = json!({
        "items": [
            {"status": "connected", "msg": "connected to server"},
            {"status": "connected", "msg": "connected to server"},
            {"status": "connected", "msg": "connected to server"},
            {"status": "connected", "msg": "connected to server"},
            {"status": "disconnected", "msg": "connected to server"}
        ]
    });

    let bytes_v1 = compress_to_bytes(&v).unwrap();

    let opt = CompressOptions { enable_value_pool: true, pool_min_repeats: 3, pool_min_string_len: 8 };
    let bytes_v2 = compress_with_options(&v, &opt).unwrap();
    let out_v2 = decompress_from_bytes(&bytes_v2).unwrap();
    assert_eq!(v, out_v2);

    assert!(bytes_v2.len() <= bytes_v1.len());
}
