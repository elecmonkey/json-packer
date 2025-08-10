use json_packer_core::{compress_to_bytes, CompressOptions};
use serde_json::json;

#[test]
fn deterministic_output_same_input() {
    let v = json!({
        "name": "Alice",
        "age": 30,
        "profile": {"name": "Alice"}
    });
    let a = compress_to_bytes(&v, &CompressOptions::default()).unwrap();
    let b = compress_to_bytes(&v, &CompressOptions::default()).unwrap();
    assert_eq!(a, b);
}
