use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Options {
  enable_value_pool: bool,
  pool_min_repeats: u32,
  pool_min_string_len: u32,
}

#[wasm_bindgen]
impl Options {
  #[wasm_bindgen(constructor)]
  pub fn new(enable_value_pool: bool, pool_min_repeats: u32, pool_min_string_len: u32) -> Options {
    Options { enable_value_pool, pool_min_repeats, pool_min_string_len }
  }
}

fn to_core_opts(o: &Options) -> json_packer::CompressOptions {
  json_packer::CompressOptions {
    enable_value_pool: o.enable_value_pool,
    pool_min_repeats: o.pool_min_repeats,
    pool_min_string_len: o.pool_min_string_len as usize,
  }
}

#[wasm_bindgen]
pub fn compress_to_bytes(json_str: &str, opts: &Options) -> Result<Box<[u8]>, JsValue> {
  let value: serde_json::Value = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&format!("invalid JSON: {e}")))?;
  let core_opts = to_core_opts(opts);
  let bytes = json_packer::compress_to_bytes(&value, &core_opts).map_err(|e| JsValue::from_str(&format!("compress error: {e}")))?;
  Ok(bytes.into_boxed_slice())
}

#[wasm_bindgen]
pub fn compress_to_base64(json_str: &str, opts: &Options) -> Result<String, JsValue> {
  let value: serde_json::Value = serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&format!("invalid JSON: {e}")))?;
  let core_opts = to_core_opts(opts);
  json_packer::compress_to_base64(&value, &core_opts).map_err(|e| JsValue::from_str(&format!("compress error: {e}")))
}

#[wasm_bindgen]
pub fn decompress_from_bytes(bytes: &[u8]) -> Result<String, JsValue> {
  let value = json_packer::decompress_from_bytes(bytes).map_err(|e| JsValue::from_str(&format!("decompress error: {e}")))?;
  serde_json::to_string(&value).map_err(|e| JsValue::from_str(&format!("stringify error: {e}")))
}

#[wasm_bindgen]
pub fn decompress_from_base64(b64: &str) -> Result<String, JsValue> {
  let value = json_packer::decompress_from_base64(b64).map_err(|e| JsValue::from_str(&format!("decompress error: {e}")))?;
  serde_json::to_string(&value).map_err(|e| JsValue::from_str(&format!("stringify error: {e}")))
}
