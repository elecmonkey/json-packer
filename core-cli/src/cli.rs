use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "json-packer-cli")]
#[command(about = "JSON compression/decompression command line tool")]
#[command(version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, global = true, help = "Show detailed output")]
    pub verbose: bool,
    
    #[arg(short, long, global = true, help = "Silent mode")]
    pub quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(alias = "c", about = "Compress JSON files or data")]
    Compress(CompressArgs),
    
    #[command(alias = "d", about = "Decompress JSON data")]
    Decompress(DecompressArgs),
    
    #[command(alias = "i", about = "View compressed file information")]
    Info(InfoArgs),
    
    #[command(alias = "b", about = "Batch process multiple files")]
    Batch(BatchArgs),
}

#[derive(Debug, Args)]
pub struct CompressArgs {
    #[arg(help = "Input file path, use '-' for stdin")]
    pub input: String,
    
    #[arg(help = "Output file path, optional, defaults to stdout")]
    pub output: Option<String>,
    
    #[arg(short, long, value_enum, default_value = "base64", help = "Output format")]
    pub format: OutputFormat,
    
    #[arg(long, help = "Enable string value pool (v2 format)")]
    pub enable_pool: bool,
    
    #[arg(long, default_value = "3", help = "Pool minimum repeat count")]
    pub pool_min_repeats: u32,
    
    #[arg(long, default_value = "8", help = "Pool minimum string length")]
    pub pool_min_string_len: usize,
    
    #[arg(long, help = "Pretty output (applies to base64 format only)")]
    pub pretty: bool,
    
    #[arg(long, help = "Show compression statistics")]
    pub stats: bool,
}

#[derive(Debug, Args)]
pub struct DecompressArgs {
    #[arg(help = "Input file path, use '-' for stdin")]
    pub input: String,
    
    #[arg(help = "Output file path, optional, defaults to stdout")]
    pub output: Option<String>,
    
    #[arg(short, long, value_enum, default_value = "auto", help = "Input format")]
    pub format: InputFormat,
    
    #[arg(long, help = "Pretty JSON output")]
    pub pretty: bool,
    
    #[arg(long, help = "Compact JSON output")]
    pub compact: bool,
}

#[derive(Debug, Args)]
pub struct InfoArgs {
    #[arg(help = "Compressed file path")]
    pub input: String,
    
    #[arg(short, long, value_enum, default_value = "auto", help = "Input format")]
    pub format: InputFormat,
    
    #[arg(long, help = "Show detailed information")]
    pub detailed: bool,
}

#[derive(Debug, Args)]
pub struct BatchArgs {
    #[arg(value_enum, help = "Operation type")]
    pub operation: BatchOperation,
    
    #[arg(help = "File pattern matching")]
    pub pattern: String,
    
    #[arg(long, help = "Output directory")]
    pub output_dir: Option<String>,
    
    #[arg(long, default_value = ".jcp", help = "Output file suffix")]
    pub output_suffix: String,
    
    #[arg(long, help = "Parallel processing count, defaults to CPU cores")]
    pub parallel: Option<usize>,
    
    #[arg(long, help = "Recursively process subdirectories")]
    pub recursive: bool,
    
    #[arg(long, help = "Enable string value pool")]
    pub enable_pool: bool,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum OutputFormat {
    #[value(name = "base64")]
    Base64,
    #[value(name = "bytes")]
    Bytes,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum InputFormat {
    #[value(name = "auto")]
    Auto,
    #[value(name = "base64")]
    Base64,
    #[value(name = "bytes")]
    Bytes,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum BatchOperation {
    #[value(name = "compress")]
    Compress,
    #[value(name = "decompress")]
    Decompress,
}