use crate::cli::InputFormat;


/// 自动检测输入格式
pub fn detect_format(data: &[u8]) -> InputFormat {
    // 检查是否为有效的Base64
    if is_base64(data) {
        InputFormat::Base64
    } else {
        InputFormat::Bytes
    }
}

/// 检查数据是否为Base64格式
fn is_base64(data: &[u8]) -> bool {
    // 检查是否都是Base64字符
    if data.is_empty() {
        return false;
    }
    
    // Base64字符集
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
    
    // 检查每个字符是否在Base64字符集中
    for &byte in data {
        if !BASE64_CHARS.contains(&byte) && !byte.is_ascii_whitespace() {
            return false;
        }
    }
    
    // 尝试解码看是否成功
    let data_str = String::from_utf8_lossy(data);
    let cleaned = data_str.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    
    json_packer::decode_base64(&cleaned).is_ok()
}

/// 格式化JSON输出
pub fn format_json(value: &serde_json::Value, pretty: bool, compact: bool) -> String {
    if pretty {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    } else if compact {
        value.to_string()
    } else {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    }
}

/// 获取格式化的文件大小字符串
pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}