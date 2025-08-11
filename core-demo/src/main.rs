use std::fs;
use json_packer::{compress_to_bytes, compress_to_base64, decompress_from_bytes, decompress_from_base64, CompressOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 读取大 JSON 文件
    println!(" - 读取测试 JSON 文件...");
    let json_content = fs::read_to_string("test_large.json")?;
    let original_size = json_content.len();
    println!("   原始 JSON 大小: {} 字节", original_size);
    
    // 2. 解析 JSON
    let json_value: serde_json::Value = serde_json::from_str(&json_content)?;
    
    // 3. 压缩到字节数组
    println!("\n - 压缩到字节数组...");
    let compressed_bytes = compress_to_bytes(&json_value, &CompressOptions::default())?;
    let compressed_size = compressed_bytes.len();
    println!("   压缩后大小: {} 字节", compressed_size);
    let compression_ratio = (compressed_size as f64 / original_size as f64) * 100.0;
    println!("   压缩率: {:.2}%", compression_ratio);
    let savings = original_size - compressed_size;
    println!("   节省空间: {} 字节", savings);
    
    // 4. 压缩到 Base64
    println!("\n - 压缩到 Base64 字符串...");
    let compressed_base64 = compress_to_base64(&json_value, &CompressOptions::default())?;
    let base64_size = compressed_base64.len();
    println!("   Base64 大小: {} 字节", base64_size);
    let base64_overhead = ((base64_size as f64 / compressed_size as f64) - 1.0) * 100.0;
    println!("   Base64 开销: {:.2}%", base64_overhead);
    
    // 5. 从字节数组解压
    println!("\n - 从字节数组解压...");
    let decompressed_from_bytes = decompress_from_bytes(&compressed_bytes)?;
    
    // 6. 从 Base64 解压
    println!(" - 从 Base64 解压...");
    let decompressed_from_base64 = decompress_from_base64(&compressed_base64)?;
    
    // 7. 验证完整性
    println!("\n - 验证数据完整性...");
    if json_value == decompressed_from_bytes {
        println!("   字节数组往返测试: 通过");
    } else {
        println!("   字节数组往返测试: 失败");
        return Err("字节数组往返测试失败".into());
    }
    
    if json_value == decompressed_from_base64 {
        println!("   Base64 往返测试: 通过");
    } else {
        println!("   Base64 往返测试: 失败");
        return Err("Base64 往返测试失败".into());
    }
    
    // 8. 统计信息
    println!("\n - 压缩统计:");
    println!("   原始 JSON:     {:>8} 字节", original_size);
    println!("   压缩后:        {:>8} 字节", compressed_size);
    println!("   Base64编码:    {:>8} 字节", base64_size);
    println!("   压缩比:        {:>7.2}%", compression_ratio);
    println!("   空间节省:      {:>8} 字节 ({:.1}%)", savings, 100.0 - compression_ratio);
    
    // 9. 显示 Base64 压缩结果的开头和结尾
    println!("\n - Base64 压缩结果预览:");
    if compressed_base64.len() > 100 {
        println!("   开头: {}...", &compressed_base64[..50]);
        println!("   结尾: ...{}", &compressed_base64[compressed_base64.len()-50..]);
    } else {
        println!("   完整: {}", compressed_base64);
    }
    
    // 10. 保存压缩结果到文件
    fs::write("test_large.output.compressed", &compressed_bytes)?;
    fs::write("test_large.output.base64", &compressed_base64)?;
    println!("\n - 压缩结果已保存:");
    println!("   二进制文件: test_large.output.compressed");
    println!("   Base64文件: test_large.output.base64");
    
    println!("\n - 压缩和解压测试全部成功!");
    
    Ok(())
}
