import fs from 'node:fs';
import addon from '../napi/index.js';
const { compressToBase64, decompressFromBase64 } = addon;

// 使用 NAPI 生成的类型定义中的 camelCase 字段名
// { enableValuePool?: boolean; poolMinRepeats?: number; poolMinStringLen?: number }
const optsOn = { enableValuePool: true, poolMinRepeats: 1, poolMinStringLen: 1 };
const optsOff = { enableValuePool: false, poolMinRepeats: 1, poolMinStringLen: 1 };

const main = () => {
  const jsonStr = fs.readFileSync(new URL('./test.json', import.meta.url), 'utf8');
  const origBytes = Buffer.byteLength(jsonStr, 'utf8');

  const b64Off = compressToBase64(jsonStr, optsOff);
  const cmpBytesOff = Buffer.from(b64Off, 'base64').length;

  const b64On = compressToBase64(jsonStr, optsOn);
  const cmpBytesOn = Buffer.from(b64On, 'base64').length;

  console.log('Original bytes:', origBytes);
  console.log('Compressed bytes (pool OFF):', cmpBytesOff, 'Base64 len:', b64Off.length, 'Ratio:', (cmpBytesOff / origBytes * 100).toFixed(2) + '%');
  console.log('Compressed bytes (pool ON): ', cmpBytesOn,  'Base64 len:', b64On.length,  'Ratio:', (cmpBytesOn  / origBytes * 100).toFixed(2) + '%');

  const outStr = decompressFromBase64(b64On);
  const equal = deepEqual(JSON.parse(outStr), JSON.parse(jsonStr));
  console.log('Roundtrip equal:', equal);
};

main();

function deepEqual(a, b) {
  if (a === b) return true; // 处理引用相等 & 基本类型相等

  if (typeof a !== typeof b) return false;
  if (a && b && typeof a === 'object') {
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;

    for (const key of keysA) {
      if (!keysB.includes(key)) return false;
      if (!deepEqual(a[key], b[key])) return false;
    }
    return true;
  }

  // 处理 NaN
  return Number.isNaN(a) && Number.isNaN(b);
}