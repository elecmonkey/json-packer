use std::path::{Path, PathBuf};
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};


use crate::cli::{BatchArgs, BatchOperation};
use crate::error::{CliError, Result};
use crate::commands::{compress, decompress};
use crate::cli::{CompressArgs, DecompressArgs, OutputFormat, InputFormat};

pub fn run(args: BatchArgs, verbose: bool, quiet: bool) -> Result<()> {
    if verbose && !quiet {
        eprintln!("Starting batch processing: {} {}", 
                 match args.operation { 
                     BatchOperation::Compress => "compress", 
                     BatchOperation::Decompress => "decompress" 
                 }, 
                 args.pattern);
    }
    
    // Find matching files
    let files = find_matching_files(&args.pattern, args.recursive)?;
    
    if files.is_empty() {
        if !quiet {
            eprintln!("Warning: No matching files found");
        }
        return Ok(());
    }
    
    if verbose && !quiet {
        eprintln!("Found {} files", files.len());
    }
    
    // Determine parallelism
    let parallel_count = args.parallel.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    });
    
    // Setup progress bar
    let progress = if !quiet {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap());
        Some(pb)
    } else {
        None
    };
    
    // Error counter
    let error_count = Arc::new(AtomicUsize::new(0));
    
    // Configure Rayon thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(parallel_count)
        .build()
        .map_err(|e| CliError::InvalidArgs(format!("Failed to create thread pool: {e}")))?;
    
    // Batch processing
    pool.install(|| {
        files.par_iter().for_each(|file| {
            let result = process_single_file(file, &args, verbose, quiet);
            
            if let Err(e) = result {
                error_count.fetch_add(1, Ordering::Relaxed);
                if !quiet {
                    eprintln!("Failed to process file {}: {}", file.display(), e);
                }
            }
            
            if let Some(ref pb) = progress {
                pb.inc(1);
                pb.set_message(format!("Processing: {}", file.file_name().unwrap_or_default().to_string_lossy()));
            }
        });
    });
    
    if let Some(ref pb) = progress {
        pb.finish_with_message("Batch processing completed");
    }
    
    let total_errors = error_count.load(Ordering::Relaxed);
    
    if total_errors > 0 {
        let message = format!("Batch processing completed, {}/{} files failed", total_errors, files.len());
        if !quiet {
            eprintln!("Warning: {message}");
        }
        return Err(CliError::BatchError { 
            failed: total_errors, 
            total: files.len() 
        });
    }
    
    if verbose && !quiet {
        eprintln!("Batch processing completed successfully, processed {} files", files.len());
    }
    
    Ok(())
}

fn find_matching_files(pattern: &str, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    // Simple glob pattern matching implementation
    if pattern.contains('*') || pattern.contains('?') {
        // Use glob pattern
        find_files_by_glob(pattern, recursive, &mut files)?;
    } else {
        // Direct path
        let path = Path::new(pattern);
        if path.is_dir() {
            find_files_in_directory(path, recursive, &mut files, "*.json")?;
        } else if path.exists() {
            files.push(path.to_path_buf());
        } else {
            return Err(CliError::FileNotFound(pattern.to_string()));
        }
    }
    
    Ok(files)
}

fn find_files_by_glob(pattern: &str, recursive: bool, files: &mut Vec<PathBuf>) -> Result<()> {
    // Simplified glob implementation - might need glob crate in actual project
    let path = Path::new(pattern);
    let dir = path.parent().unwrap_or(Path::new("."));
    let filename_pattern = path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("*");
    
    find_files_in_directory(dir, recursive, files, filename_pattern)?;
    Ok(())
}

fn find_files_in_directory(
    dir: &Path, 
    recursive: bool, 
    files: &mut Vec<PathBuf>, 
    pattern: &str
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if matches_pattern(&path, pattern) {
                files.push(path);
            }
        } else if path.is_dir() && recursive {
            find_files_in_directory(&path, recursive, files, pattern)?;
        }
    }
    
    Ok(())
}

fn matches_pattern(path: &Path, pattern: &str) -> bool {
    let filename = path.file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("");
    
    if pattern == "*" {
        return true;
    }
    
    if let Some(ext) = pattern.strip_prefix("*.") {
        return path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e == ext)
            .unwrap_or(false);
    }
    
    // Simple string matching
    filename.contains(pattern.trim_matches('*'))
}

fn process_single_file(
    file: &Path,
    args: &BatchArgs,
    verbose: bool,
    _quiet: bool
) -> Result<()> {
    let input_path = file.to_string_lossy().to_string();
    
    let output_path = determine_output_path(file, args)?;
    
    match args.operation {
        BatchOperation::Compress => {
            let compress_args = CompressArgs {
                input: input_path,
                output: Some(output_path),
                format: OutputFormat::Base64,
                enable_pool: args.enable_pool,
                pool_min_repeats: 3,
                pool_min_string_len: 8,
                pretty: false,
                stats: false,
            };
            
            compress::run(compress_args, verbose, true) // 强制quiet模式避免大量输出
        }
        BatchOperation::Decompress => {
            let decompress_args = DecompressArgs {
                input: input_path,
                output: Some(output_path),
                format: InputFormat::Auto,
                pretty: false,
                compact: false,
            };
            
            decompress::run(decompress_args, verbose, true) // 强制quiet模式避免大量输出
        }
    }
}

fn determine_output_path(file: &Path, args: &BatchArgs) -> Result<String> {
    let file_stem = file.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| CliError::InvalidArgs("Invalid file name".to_string()))?;
    
    let output_dir = if let Some(ref dir) = args.output_dir {
        Path::new(dir)
    } else {
        file.parent().unwrap_or(Path::new("."))
    };
    
    // Ensure output directory exists
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }
    
    let output_filename = match args.operation {
        BatchOperation::Compress => {
            format!("{}{}", file_stem, args.output_suffix)
        }
        BatchOperation::Decompress => {
            if file_stem.ends_with(".jcp") {
                file_stem.trim_end_matches(".jcp").to_string() + ".json"
            } else {
                format!("{file_stem}_decompressed.json")
            }
        }
    };
    
    let output_path = output_dir.join(output_filename);
    Ok(output_path.to_string_lossy().to_string())
}