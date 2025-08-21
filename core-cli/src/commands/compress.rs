use json_packer::{compress_to_bytes, compress_to_base64, CompressOptions};
use serde_json::Value;

use crate::cli::{CompressArgs, OutputFormat};
use crate::error::Result;
use crate::utils::{io, stats};

pub fn run(args: CompressArgs, verbose: bool, quiet: bool) -> Result<()> {
    if verbose && !quiet {
        eprintln!("Starting compression: {}", args.input);
    }
    
    // Read input JSON
    let input_content = io::read_input(&args.input)?;
    let json_value: Value = serde_json::from_str(&input_content)?;
    
    // Build compression options
    let compress_opts = CompressOptions {
        enable_value_pool: args.enable_pool,
        pool_min_repeats: args.pool_min_repeats,
        pool_min_string_len: args.pool_min_string_len,
    };
    
    if verbose && !quiet {
        eprintln!("Compression options: {compress_opts:?}");
        
        // Show JSON analysis information
        let analysis = stats::analyze_json_value(&json_value);
        analysis.print();
        eprintln!();
    }
    
    // Execute compression
    let result = match args.format {
        OutputFormat::Base64 => {
            let compressed = compress_to_base64(&json_value, &compress_opts)?;
            if args.pretty {
                // Add line breaks to Base64 for readability
                format_base64_pretty(&compressed)
            } else {
                compressed
            }
        }
        OutputFormat::Bytes => {
            let compressed_bytes = compress_to_bytes(&json_value, &compress_opts)?;
            // For byte output, we output hex representation or write directly to file
            if args.output.is_some() {
                // If there's an output file, write bytes directly
                io::write_output_bytes(args.output.as_deref(), &compressed_bytes)?;
                
                if args.stats && !quiet {
                    let original_size = input_content.len() as u64;
                    let compressed_size = compressed_bytes.len() as u64;
                    let stats = stats::CompressionStats::new(original_size, compressed_size);
                    stats.print();
                }
                
                if verbose && !quiet {
                    eprintln!("Compression completed, written to: {}", args.output.as_deref().unwrap());
                }
                return Ok(());
            } else {
                // If outputting to stdout, use hex format
                hex::encode(compressed_bytes)
            }
        }
    };
    
    // Output results
    io::write_output(args.output.as_deref(), &result)?;
    
    // Show statistics
    if args.stats && !quiet {
        let original_size = input_content.len() as u64;
        let compressed_size = match args.format {
            OutputFormat::Base64 => result.len() as u64,
            OutputFormat::Bytes => result.len() as u64 / 2, // Number of bytes in hex representation
        };
        let stats = stats::CompressionStats::new(original_size, compressed_size);
        stats.print();
    }
    
    if verbose && !quiet {
        match &args.output {
            Some(output) => eprintln!("Compression completed, written to: {output}"),
            None => eprintln!("Compression completed, output to stdout"),
        }
    }
    
    Ok(())
}

fn format_base64_pretty(base64: &str) -> String {
    const LINE_LENGTH: usize = 76;
    let mut result = String::new();
    
    for (i, char) in base64.chars().enumerate() {
        if i > 0 && i % LINE_LENGTH == 0 {
            result.push('\n');
        }
        result.push(char);
    }
    
    result
}