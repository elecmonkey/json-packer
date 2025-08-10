use proptest::prelude::*;

use json_packer_core::{compress_to_bytes, decompress_from_bytes};

fn arb_json() -> impl Strategy<Value = serde_json::Value> {
    // Build a JSON generator without NaN/Inf
    let leaf = prop_oneof![
        Just(serde_json::Value::Null),
        any::<bool>().prop_map(serde_json::Value::Bool),
        // integers within full i64/u64 via Number from i64/u64
        any::<i64>().prop_map(|x| serde_json::Value::Number(x.into())),
        any::<u64>().prop_map(|x| serde_json::Value::Number(serde_json::Number::from(x))),
        // finite f64 only
        any::<f64>()
            .prop_filter("finite", |f| f.is_finite())
            .prop_map(|f| serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap())),
        // short strings
        "[ -~]{0,64}".prop_map(|s| serde_json::Value::String(s)),
    ];

    leaf.prop_recursive(4, 64, 10, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 0..8).prop_map(serde_json::Value::Array),
            prop::collection::hash_map("[a-zA-Z0-9_]{1,12}", inner, 0..8)
                .prop_map(|m| serde_json::Value::Object(m.into_iter().collect()))
        ]
    })
}

proptest! {
    #[test]
    fn prop_roundtrip(v in arb_json()) {
        let bytes = compress_to_bytes(&v).unwrap();
        let out = decompress_from_bytes(&bytes).unwrap();
        prop_assert_eq!(v, out);
    }
}
