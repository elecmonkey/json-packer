use json_packer_core as core;
use json_packer_core::CompressOptions;
use napi::bindgen_prelude::*;
use napi_derive::napi;

fn to_core_opts(opts: &Options) -> CompressOptions {
  CompressOptions {
    enable_value_pool: opts.enable_value_pool.unwrap_or(false),
    pool_min_repeats: opts.pool_min_repeats.unwrap_or(3),
    pool_min_string_len: opts.pool_min_string_len.unwrap_or(8) as usize,
  }
}

#[napi(object)]
pub struct Options {
  pub enable_value_pool: Option<bool>,
  pub pool_min_repeats: Option<u32>,
  pub pool_min_string_len: Option<u32>,
}

#[napi]
pub fn compress_to_bytes(json_str: String, opts: Options) -> Result<Buffer> {
  let value: serde_json::Value = serde_json::from_str(&json_str)
    .map_err(|e| Error::new(Status::InvalidArg, format!("invalid JSON: {e}")))?;
  let bytes = core::compress_to_bytes(&value, &to_core_opts(&opts))
    .map_err(|e| Error::new(Status::GenericFailure, format!("compress error: {e}")))?;
  Ok(Buffer::from(bytes))
}

#[napi]
pub fn compress_to_base64(json_str: String, opts: Options) -> Result<String> {
  let value: serde_json::Value = serde_json::from_str(&json_str)
    .map_err(|e| Error::new(Status::InvalidArg, format!("invalid JSON: {e}")))?;
  core::compress_to_base64(&value, &to_core_opts(&opts))
    .map_err(|e| Error::new(Status::GenericFailure, format!("compress error: {e}")))
}

#[napi]
pub fn decompress_from_bytes(bytes: Buffer) -> Result<String> {
  let value = core::decompress_from_bytes(&bytes)
    .map_err(|e| Error::new(Status::GenericFailure, format!("decompress error: {e}")))?;
  serde_json::to_string(&value)
    .map_err(|e| Error::new(Status::GenericFailure, format!("stringify error: {e}")))
}

#[napi]
pub fn decompress_from_base64(b64: String) -> Result<String> {
  let value = core::decompress_from_base64(&b64)
    .map_err(|e| Error::new(Status::GenericFailure, format!("decompress error: {e}")))?;
  serde_json::to_string(&value)
    .map_err(|e| Error::new(Status::GenericFailure, format!("stringify error: {e}")))
}
