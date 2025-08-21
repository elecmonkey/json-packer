use json_packer::decompress_from_bytes;
use json_packer::test_expose::{BitReader, read_header};

use crate::cli::{InfoArgs, InputFormat};
use crate::error::{CliError, Result};
use crate::utils::{io, format, stats};

pub fn run(args: InfoArgs, verbose: bool, quiet: bool) -> Result<()> {
    if verbose && !quiet {
        eprintln!("Analyzing file: {}", args.input);
    }
    
    // Read input data
    let input_data = if args.format == InputFormat::Bytes || 
                      (args.format == InputFormat::Auto && is_binary_file(&args.input)) {
        io::read_input_bytes(&args.input)?
    } else {
        io::read_input(&args.input)?.into_bytes()
    };
    
    // Parse input format
    let compressed_bytes = match args.format {
        InputFormat::Auto => {
            let detected_format = format::detect_format(&input_data);
            parse_input_for_info(&input_data, detected_format)?
        }
        format => parse_input_for_info(&input_data, format)?
    };
    
    // Analyze compressed file
    let file_info = analyze_compressed_file(&compressed_bytes)?;
    
    // Show information
    file_info.print(args.detailed);
    
    if args.detailed {
        // Try to decompress and analyze original data
        if let Ok(decompressed) = decompress_from_bytes(&compressed_bytes) {
            eprintln!("\nOriginal JSON structure analysis:");
            let analysis = stats::analyze_json_value(&decompressed);
            analysis.print();
        }
    }
    
    Ok(())
}

fn parse_input_for_info(data: &[u8], format: InputFormat) -> Result<Vec<u8>> {
    match format {
        InputFormat::Base64 => {
            let data_str = String::from_utf8(data.to_vec())
                .map_err(|e| CliError::InvalidFormat(format!("Invalid UTF-8 in Base64 input: {e}")))?;
            
            json_packer::decode_base64(&data_str)
                .map_err(|e| CliError::InvalidFormat(format!("Invalid Base64: {e}")))
        }
        InputFormat::Bytes => {
            Ok(data.to_vec())
        }
        InputFormat::Auto => {
            unreachable!("Auto format should be resolved before calling this function")
        }
    }
}

fn analyze_compressed_file(data: &[u8]) -> Result<stats::FileInfo> {
    // Parse file header
    let mut reader = BitReader::new(data);
    let header = read_header(&mut reader)
        .map_err(CliError::Compression)?;
    
    let version = match header.version {
        1 => "v1 (no value pool)".to_string(),
        2 => "v2 (value pool enabled)".to_string(),
        v => format!("v{v} (unknown version)"),
    };
    
    // Try to decompress to estimate original size
    let (original_size_estimate, compression_ratio) = 
        if let Ok(decompressed) = decompress_from_bytes(data) {
            let original_json = serde_json::to_string(&decompressed)
                .unwrap_or_default();
            let original_size = original_json.len() as u64;
            let compressed_size = data.len() as u64;
            let ratio = if original_size > 0 {
                Some(compressed_size as f64 / original_size as f64)
            } else {
                None
            };
            (Some(original_size), ratio)
        } else {
            (None, None)
        };
    
    let file_info = stats::FileInfo {
        version,
        original_size_estimate,
        compressed_size: data.len() as u64,
        dictionary_size: if header.dict_len > 0 { Some(header.dict_len as usize) } else { None },
        pool_size: if header.pool_len > 0 { Some(header.pool_len as usize) } else { None },
        compression_ratio,
    };
    
    Ok(file_info)
}

fn is_binary_file(path: &str) -> bool {
    if path == "-" {
        return false; // stdin assumed to be text
    }
    
    // Check file extension
    let path = std::path::Path::new(path);
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "bin" | "dat" | "jcp")
    } else {
        false
    }
}