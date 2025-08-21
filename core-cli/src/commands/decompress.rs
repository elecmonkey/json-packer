use json_packer::{decompress_from_bytes, decompress_from_base64};

use crate::cli::{DecompressArgs, InputFormat};
use crate::error::{CliError, Result};
use crate::utils::{io, format};

pub fn run(args: DecompressArgs, verbose: bool, quiet: bool) -> Result<()> {
    if verbose && !quiet {
        eprintln!("Starting decompression: {}", args.input);
    }
    
    // Read input data
    let input_data = if args.format == InputFormat::Bytes || 
                      (args.format == InputFormat::Auto && is_binary_file(&args.input)) {
        io::read_input_bytes(&args.input)?
    } else {
        io::read_input(&args.input)?.into_bytes()
    };
    
    // Parse input according to format
    let decompressed_value = match args.format {
        InputFormat::Auto => {
            // Auto-detect format
            let detected_format = format::detect_format(&input_data);
            decompress_with_format(&input_data, detected_format, verbose, quiet)?
        }
        format => decompress_with_format(&input_data, format, verbose, quiet)?
    };
    
    // Format JSON output
    let output_content = if args.compact {
        format::format_json(&decompressed_value, false, true)
    } else {
        format::format_json(&decompressed_value, args.pretty, false)
    };
    
    // Output results
    io::write_output(args.output.as_deref(), &output_content)?;
    
    if verbose && !quiet {
        match &args.output {
            Some(output) => eprintln!("Decompression completed, written to: {output}"),
            None => eprintln!("Decompression completed, output to stdout"),
        }
    }
    
    Ok(())
}

fn decompress_with_format(
    data: &[u8], 
    format: InputFormat, 
    verbose: bool, 
    quiet: bool
) -> Result<serde_json::Value> {
    match format {
        InputFormat::Base64 => {
            if verbose && !quiet {
                eprintln!("Using Base64 format for decompression");
            }
            let data_str = String::from_utf8(data.to_vec())
                .map_err(|e| CliError::InvalidFormat(format!("Invalid UTF-8 in Base64 input: {e}")))?;
            
            decompress_from_base64(&data_str)
                .map_err(CliError::Compression)
        }
        InputFormat::Bytes => {
            if verbose && !quiet {
                eprintln!("Using byte format for decompression");
            }
            decompress_from_bytes(data)
                .map_err(CliError::Compression)
        }
        InputFormat::Auto => {
            // This should not happen as Auto is resolved before calling this function
            unreachable!("Auto format should be resolved before calling this function")
        }
    }
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