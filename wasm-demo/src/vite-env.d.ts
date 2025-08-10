/// <reference types="vite/client" />

declare module 'json_packer_wasm' {
  // wasm-pack 默认导出初始化函数
  type InitInput = {
    module_or_path?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module
  } | undefined
  const init: (input?: InitInput) => Promise<void>
  export default init

  export class Options {
    constructor(enable_value_pool: boolean, pool_min_repeats: number, pool_min_string_len: number)
  }

  export function compress_to_bytes(json_str: string, opts: Options): Uint8Array
  export function compress_to_base64(json_str: string, opts: Options): string
  export function decompress_from_bytes(bytes: Uint8Array): string
  export function decompress_from_base64(b64: string): string
}