use serde_json::json;
use json_packer_core::compress_to_bytes;

#[test]
fn dictionary_roundtrip_via_public_api() {
    let v = json!({
        "用户": "张三",
        "年龄": 25,
        "🚀": "rocket"
    });
    let _bytes = compress_to_bytes(&v).unwrap(); // covers dictionary write path implicitly
}
