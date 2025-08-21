use serde_json::Value;
use std::collections::HashMap;
use crate::utils::format::format_size;

#[derive(Debug)]
pub struct CompressionStats {
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub space_saved: u64,
    pub space_saved_percent: f64,
}

impl CompressionStats {
    pub fn new(original_size: u64, compressed_size: u64) -> Self {
        let compression_ratio = if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            0.0
        };
        
        let space_saved = original_size.saturating_sub(compressed_size);
        
        let space_saved_percent = if original_size > 0 {
            (space_saved as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };
        
        Self {
            original_size,
            compressed_size,
            compression_ratio,
            space_saved,
            space_saved_percent,
        }
    }
    
    pub fn print(&self) {
        println!("Compression Statistics:");
        println!("  Original size: {}", format_size(self.original_size));
        println!("  Compressed size: {}", format_size(self.compressed_size));
        println!("  Compression ratio: {:.2}%", (1.0 - self.compression_ratio) * 100.0);
        println!("  Space saved: {} ({:.2}%)", format_size(self.space_saved), self.space_saved_percent);
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub version: String,
    pub original_size_estimate: Option<u64>,
    pub compressed_size: u64,
    pub dictionary_size: Option<usize>,
    pub pool_size: Option<usize>,
    pub compression_ratio: Option<f64>,
}

impl FileInfo {
    pub fn print(&self, detailed: bool) {
        println!("File Information:");
        println!("  Version: {}", self.version);
        println!("  Compressed size: {}", format_size(self.compressed_size));
        
        if let Some(original) = self.original_size_estimate {
            println!("  Original size estimate: {}", format_size(original));
            if let Some(ratio) = self.compression_ratio {
                println!("  Compression ratio: {:.2}%", (1.0 - ratio) * 100.0);
            }
        }
        
        if detailed {
            if let Some(dict_size) = self.dictionary_size {
                println!("  Dictionary size: {dict_size} keys");
            }
            if let Some(pool_size) = self.pool_size {
                println!("  Value pool size: {pool_size} strings");
            }
        }
    }
}

/// 分析JSON值的基本统计信息
pub fn analyze_json_value(value: &Value) -> JsonAnalysis {
    let mut analysis = JsonAnalysis::new();
    analyze_value_recursive(value, &mut analysis);
    analysis
}

#[derive(Debug)]
pub struct JsonAnalysis {
    pub total_nodes: u64,
    pub null_count: u64,
    pub bool_count: u64,
    pub number_count: u64,
    pub string_count: u64,
    pub array_count: u64,
    pub object_count: u64,
    pub max_depth: u64,
    pub string_chars: u64,
    pub unique_keys: HashMap<String, u64>,
    pub unique_strings: HashMap<String, u64>,
}

impl JsonAnalysis {
    fn new() -> Self {
        Self {
            total_nodes: 0,
            null_count: 0,
            bool_count: 0,
            number_count: 0,
            string_count: 0,
            array_count: 0,
            object_count: 0,
            max_depth: 0,
            string_chars: 0,
            unique_keys: HashMap::new(),
            unique_strings: HashMap::new(),
        }
    }
    
    pub fn print(&self) {
        println!("JSON Structure Analysis:");
        println!("  Total nodes: {}", self.total_nodes);
        println!("  Max depth: {}", self.max_depth);
        println!("  Null values: {}", self.null_count);
        println!("  Boolean values: {}", self.bool_count);
        println!("  Numbers: {}", self.number_count);
        println!("  Strings: {} (total characters: {})", self.string_count, self.string_chars);
        println!("  Arrays: {}", self.array_count);
        println!("  Objects: {}", self.object_count);
        println!("  Unique keys: {}", self.unique_keys.len());
        println!("  Unique strings: {}", self.unique_strings.len());
        
        // Show repeated strings
        let repeated_strings: Vec<_> = self.unique_strings.iter()
            .filter(|(_, &count)| count > 1)
            .collect();
        
        if !repeated_strings.is_empty() {
            println!("  Repeated strings (suitable for value pool optimization):");
            for (string, count) in repeated_strings.iter().take(5) {
                println!("    \"{string}\" × {count}");
            }
            if repeated_strings.len() > 5 {
                println!("    ... and {} more repeated strings", repeated_strings.len() - 5);
            }
        }
    }
}

fn analyze_value_recursive(value: &Value, analysis: &mut JsonAnalysis) {
    analyze_value_recursive_with_depth(value, analysis, 0);
}

fn analyze_value_recursive_with_depth(value: &Value, analysis: &mut JsonAnalysis, depth: u64) {
    analysis.total_nodes += 1;
    analysis.max_depth = analysis.max_depth.max(depth);
    
    match value {
        Value::Null => analysis.null_count += 1,
        Value::Bool(_) => analysis.bool_count += 1,
        Value::Number(_) => analysis.number_count += 1,
        Value::String(s) => {
            analysis.string_count += 1;
            analysis.string_chars += s.len() as u64;
            *analysis.unique_strings.entry(s.clone()).or_insert(0) += 1;
        }
        Value::Array(arr) => {
            analysis.array_count += 1;
            for item in arr {
                analyze_value_recursive_with_depth(item, analysis, depth + 1);
            }
        }
        Value::Object(obj) => {
            analysis.object_count += 1;
            for (key, val) in obj {
                *analysis.unique_keys.entry(key.clone()).or_insert(0) += 1;
                analyze_value_recursive_with_depth(val, analysis, depth + 1);
            }
        }
    }
}