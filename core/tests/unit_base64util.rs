use json_packer_core::{encode_base64, decode_base64};
use base64::Engine; // bring trait for STANDARD.encode

#[test]
fn base64_standard_and_no_pad_decode() {
    let data = b"hello world";
    let b64_no_pad = encode_base64(data);
    // produce a padded variant and ensure decoding works too
    let b64_padded = base64::engine::general_purpose::STANDARD.encode(data);

    let d1 = decode_base64(&b64_no_pad).unwrap();
    let d2 = decode_base64(&b64_padded).unwrap();
    assert_eq!(d1, data);
    assert_eq!(d2, data);
}
