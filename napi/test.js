const { compressToBase64, decompressFromBase64 } = require('./index.js');
const { deepEqual } = require('assert');

// 测试数据
const testData = {
  name: "John Doe",
  age: 30,
  city: "New York",
  hobbies: ["reading", "swimming", "coding"],
  address: {
    street: "123 Main St",
    zipcode: "10001"
  }
};

const jsonString = JSON.stringify(testData);

try {
  // 测试压缩和解压缩
  const compressed = compressToBase64(jsonString, { enableValuePool: true });
  const decompressed = decompressFromBase64(compressed, { enableValuePool: true });
  
  // 验证数据一致性
  const parsedDecompressed = JSON.parse(decompressed);
  deepEqual(parsedDecompressed, testData);
  
  console.log('✅ NAPI bindings test passed!');
  console.log(`Original size: ${jsonString.length} bytes`);
  console.log(`Compressed size: ${compressed.length} bytes`);
  console.log(`Compression ratio: ${(compressed.length / jsonString.length * 100).toFixed(2)}%`);
  
  process.exit(0);
} catch (error) {
  console.error('❌ NAPI bindings test failed:', error);
  process.exit(1);
}