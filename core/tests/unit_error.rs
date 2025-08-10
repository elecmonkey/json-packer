use json_packer_core::Error;

#[test]
fn error_variants_debug() {
    // ensure variants exist and are constructible
    let _ = Error::IllegalFloat;
    let _ = Error::BitstreamOutOfBounds;
    let _ = Error::VarintError;
    let _ = Error::BadMagic;
    let _ = Error::BadVersion;
    let _ = Error::HuffmanError;
}
