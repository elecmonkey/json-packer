use base64::engine::general_purpose::{STANDARD, STANDARD_NO_PAD};
use base64::Engine as _;

use crate::Error;

pub fn encode_base64(bytes: &[u8]) -> String {
    STANDARD_NO_PAD.encode(bytes)
}

pub fn decode_base64(s: &str) -> Result<Vec<u8>, Error> {
    match STANDARD_NO_PAD.decode(s) {
        Ok(v) => Ok(v),
        Err(_) => Ok(STANDARD.decode(s)?),
    }
}

