mod error;
mod base64util;
mod types;
mod bitstream;
mod varint;
mod header;
mod dict;
mod huffman;
mod encode;
mod decode;
mod pool;

pub use error::Error;
pub use base64util::{encode_base64, decode_base64};
pub use dict::{collect_keys, write_dictionary, read_dictionary};
pub use huffman::HuffmanCodec;
pub use pool::{StringPool, PoolConfig};
pub use encode::CompressOptions;

#[doc(hidden)]
pub use header::{VERSION_V1, VERSION_V2};

#[doc(hidden)]
pub mod test_expose {
    pub use crate::bitstream::{BitReader, BitWriter};
    pub use crate::varint::{read_sleb128, read_uleb128, write_sleb128, write_uleb128};
    pub use crate::header::{read_header, write_header, MAGIC, VERSION_V1, VERSION_V2};
    pub use crate::dict::{collect_keys, read_dictionary, write_dictionary};
    pub use crate::types::tag;
    pub use crate::pool::{collect_string_pool, write_string_pool};
    pub use crate::encode::{compress_with_options, CompressOptions};
}


/// 压缩 JSON 到字节数组（无状态，按调用传入选项）
pub fn compress_to_bytes(value: &serde_json::Value, opts: &CompressOptions) -> Result<Vec<u8>, Error> {
    encode::compress_with_options(value, opts)
}

/// 压缩 JSON 到 Base64 字符串（无状态，按调用传入选项）
pub fn compress_to_base64(value: &serde_json::Value, opts: &CompressOptions) -> Result<String, Error> {
    let bytes = compress_to_bytes(value, opts)?;
    Ok(encode_base64(&bytes))
}

/// 从字节数组解压为 JSON
pub fn decompress_from_bytes(bytes: &[u8]) -> Result<serde_json::Value, Error> { decode::decompress_from_bytes(bytes) }

/// 从 Base64 字符串解压为 JSON
pub fn decompress_from_base64(s: &str) -> Result<serde_json::Value, Error> {
    let bytes = decode_base64(s)?;
    decompress_from_bytes(&bytes)
}