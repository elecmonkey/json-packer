use json_packer_core::{compress_to_base64, decompress_from_base64, CompressOptions};
use serde_json::json;

#[test]
fn base64_roundtrip_small_object() {
    let v = json!({"ok": true, "count": 42});
    let b64 = compress_to_base64(&v, &CompressOptions::default()).unwrap();
    let out = decompress_from_base64(&b64).unwrap();
    assert_eq!(v, out);
}
