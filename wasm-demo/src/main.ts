import init, { compress_to_base64, decompress_from_base64, Options } from 'json_packer_wasm'
import wasmUrl from 'json_packer_wasm/json_packer_wasm_bg.wasm?url'

async function initApp() {
  // 先初始化 WASM
  try {
    console.log('Loading WASM from:', wasmUrl)
    await init({ module_or_path: wasmUrl })
  } catch (e) {
    console.error('WASM initialization failed:', e)
    document.querySelector<HTMLDivElement>('#app')!.innerHTML = `<div style="color: red; padding: 20px;">WASM 加载失败: ${e}</div>`
    return
  }

  const app = document.querySelector<HTMLDivElement>('#app')!
  app.innerHTML = `
    <div style="font-family: system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif; line-height: 1.5; max-width: 900px; margin: 40px auto; padding: 16px;">
      <h1 style="margin: 0 0 12px;">JSON Packer (WASM) Demo</h1>
      <p style="margin: 0 0 16px; color: #555;">Toggle string pool options and compare compression sizes. Try your own JSON.</p>

      <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 12px;">
        <div>
          <label style="display:block; font-size: 12px; color:#666; margin-bottom:4px;">Input JSON</label>
          <textarea id="json-input" style="width:100%; height:240px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; font-size:12px; padding:8px; border:1px solid #ddd; border-radius:8px;">{"meta":{"title":"Demo","version":"1.0.0"},"users":[{"id":1,"name":"Alice","status":"active","logs":["connected to server","connected to server","connected to server"]},{"id":2,"name":"Bob","status":"active","logs":["connected to server","connected to server","connected to server"]},{"id":3,"name":"Carol","status":"inactive","logs":["connected to server","connected to server","connected to server"]}]}</textarea>
        </div>
        <div>
          <label style="display:block; font-size: 12px; color:#666; margin-bottom:4px;">Compressed (Base64)</label>
          <textarea id="b64-output" readonly style="width:100%; height:240px; font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace; font-size:12px; padding:8px; border:1px solid #ddd; border-radius:8px; background:#fafafa;"></textarea>
        </div>
      </div>

      <div style="display:flex; gap:12px; flex-wrap: wrap; align-items: center; margin-bottom: 8px;">
        <label style="display:flex; align-items:center; gap:8px;"> 
          <input type="checkbox" id="pool-enabled" checked /> Enable value pool
        </label>
        <label style="display:flex; align-items:center; gap:8px;">Min repeats <input id="pool-repeats" type="number" min="1" value="3" style="width:72px; padding:4px 6px; border:1px solid #ddd; border-radius:6px;"/></label>
        <label style="display:flex; align-items:center; gap:8px;">Min string len <input id="pool-len" type="number" min="1" value="6" style="width:72px; padding:4px 6px; border:1px solid #ddd; border-radius:6px;"/></label>
      </div>
      
      <div style="display:flex; gap:12px; flex-wrap: wrap; align-items: center; margin-bottom: 8px;">
        <button id="btn-compress" style="padding:8px 12px; border:1px solid #0ea5e9; background:#0ea5e9; color:#fff; border-radius:8px;">Compress</button>
        <button id="btn-decompress" style="padding:8px 12px; border:1px solid #111827; background:#111827; color:#fff; border-radius:8px;">Decompress</button>
        <button id="btn-compress-bench" style="padding:8px 12px; border:1px solid #7e22ce; background:#7e22ce; color:#fff; border-radius:8px;">Compress 1000x</button>
        <button id="btn-decompress-bench" style="padding:8px 12px; border:1px solid #ea580c; background:#ea580c; color:#fff; border-radius:8px;">Decompress 1000x</button>
        <label style="display:flex; align-items:center; gap:8px;">Repeat times <input id="repeat-times" type="number" min="1" value="1000" style="width:80px; padding:4px 6px; border:1px solid #ddd; border-radius:6px;"/></label>
      </div>

      <div id="stats" style="font-size:13px; color:#333; margin-bottom: 8px;"></div>
      <div id="msg" style="font-size:13px; color:#b91c1c;"></div>
    </div>
  `

  function readInputs() {
    const json = (document.getElementById('json-input') as HTMLTextAreaElement).value
    const enable = (document.getElementById('pool-enabled') as HTMLInputElement).checked
    const repeats = parseInt((document.getElementById('pool-repeats') as HTMLInputElement).value || '3', 10)
    const minLen = parseInt((document.getElementById('pool-len') as HTMLInputElement).value || '6', 10)
    return { json, enable, repeats, minLen }
  }

  function readRepeatTimes() {
    const repeatTimes = parseInt((document.getElementById('repeat-times') as HTMLInputElement).value || '1000', 10)
    return Math.max(1, repeatTimes) // 至少重复1次
  }

  function updateButtonLabels() {
    const repeatTimes = readRepeatTimes();
    const compressBtn = document.getElementById('btn-compress-bench') as HTMLButtonElement;
    const decompressBtn = document.getElementById('btn-decompress-bench') as HTMLButtonElement;
    
    if (compressBtn) compressBtn.textContent = `Compress ${repeatTimes}x`;
    if (decompressBtn) decompressBtn.textContent = `Decompress ${repeatTimes}x`;
  }

  function setMsg(text: string, isError = false) {
    const el = document.getElementById('msg')!
    el.style.color = isError ? '#b91c1c' : '#065f46'
    el.textContent = text
  }

  function setStats(text: string) {
    document.getElementById('stats')!.textContent = text
  }

  function setB64(text: string) {
    ;(document.getElementById('b64-output') as HTMLTextAreaElement).value = text
  }

  async function handleCompress() {
    try {
      setMsg('')
      const { json, enable, repeats, minLen } = readInputs()

      const opts = new Options(enable, repeats >>> 0, minLen >>> 0)
      const start = performance.now()
      const b64 = compress_to_base64(json, opts)
      const end = performance.now()
      setB64(b64)

      // 统计
      const originalBytes = new TextEncoder().encode(json).length
      const b64Bytes = new TextEncoder().encode(b64).length
      const ratio = ((b64Bytes / originalBytes) * 100).toFixed(2)
      setStats(`Original: ${originalBytes} bytes, Base64: ${b64Bytes} bytes (${ratio}%)`)
      setMsg(`Compression completed in ${(end - start).toFixed(2)}ms`, false)
    } catch (e: any) {
      setMsg(e?.message || String(e), true)
    }
  }

  async function handleDecompress() {
    try {
      setMsg('')
      const b64 = (document.getElementById('b64-output') as HTMLTextAreaElement).value
      if (!b64.trim()) {
        setMsg('Please compress some JSON first', true)
        return
      }
      const start = performance.now()
      const json = decompress_from_base64(b64)
      const end = performance.now()
      ;(document.getElementById('json-input') as HTMLTextAreaElement).value = json
      setMsg(`Decompression completed in ${(end - start).toFixed(2)}ms`, false)
    } catch (e: any) {
      setMsg(e?.message || String(e), true)
    }
  }

  async function handleCompressBench() {
    try {
      setMsg('')
      const { json, enable, repeats, minLen } = readInputs()
      const repeatTimes = readRepeatTimes()
      const opts = new Options(enable, repeats >>> 0, minLen >>> 0)
      
      // 执行N次压缩并计时
      const start = performance.now()
      let b64 = ''
      for (let i = 0; i < repeatTimes; i++) {
        b64 = compress_to_base64(json, opts)
      }
      const end = performance.now()
      
      setB64(b64)
      
      // 统计
      const originalBytes = new TextEncoder().encode(json).length
      const b64Bytes = new TextEncoder().encode(b64).length
      const ratio = ((b64Bytes / originalBytes) * 100).toFixed(2)
      setStats(`Original: ${originalBytes} bytes, Base64: ${b64Bytes} bytes (${ratio}%)`)
      setMsg(`Compression ${repeatTimes}x completed in ${(end - start).toFixed(2)}ms (${((end - start) / repeatTimes).toFixed(4)}ms per op)`, false)
    } catch (e: any) {
      setMsg(e?.message || String(e), true)
    }
  }

  async function handleDecompressBench() {
    try {
      setMsg('')
      const b64 = (document.getElementById('b64-output') as HTMLTextAreaElement).value
      if (!b64.trim()) {
        setMsg('Please compress some JSON first', true)
        return
      }
      const repeatTimes = readRepeatTimes()
      
      // 执行N次解压缩并计时
      const start = performance.now()
      let json = ''
      for (let i = 0; i < repeatTimes; i++) {
        json = decompress_from_base64(b64)
      }
      const end = performance.now()
      
      ;(document.getElementById('json-input') as HTMLTextAreaElement).value = json
      setMsg(`Decompression ${repeatTimes}x completed in ${(end - start).toFixed(2)}ms (${((end - start) / repeatTimes).toFixed(4)}ms per op)`, false)
    } catch (e: any) {
      setMsg(e?.message || String(e), true)
    }
  }

  // 绑定事件（确保元素已渲染且类型正确）
  const compressEl = document.getElementById('btn-compress')
  const decompressEl = document.getElementById('btn-decompress')
  const compressBenchEl = document.getElementById('btn-compress-bench')
  const decompressBenchEl = document.getElementById('btn-decompress-bench')
  const repeatTimesEl = document.getElementById('repeat-times') as HTMLInputElement

  if (compressEl instanceof HTMLButtonElement && decompressEl instanceof HTMLButtonElement) {
    compressEl.addEventListener('click', handleCompress)
    decompressEl.addEventListener('click', handleDecompress)
    
    // 绑定新按钮事件
    if (compressBenchEl instanceof HTMLButtonElement && decompressBenchEl instanceof HTMLButtonElement) {
      compressBenchEl.addEventListener('click', handleCompressBench)
      decompressBenchEl.addEventListener('click', handleDecompressBench)
    }
    
    // 绑定重复次数输入框事件
    if (repeatTimesEl) {
      repeatTimesEl.addEventListener('input', updateButtonLabels);
    }
    
    // 首次渲染后自动压缩一次作为演示
    handleCompress()
  } else {
    setMsg('Failed to bind event listeners: button element not found or wrong type', true)
    console.error('bind-failed', { compressEl, decompressEl })
  }
  
  // 初始化按钮标签
  updateButtonLabels();
}

// 启动应用
initApp()