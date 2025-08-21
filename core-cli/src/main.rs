mod cli;
mod error;
mod commands;
mod utils;

use clap::Parser;
use error::Result;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    
    if let Err(err) = run(cli) {
        eprintln!("Error: {err}");
        std::process::exit(err.exit_code());
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Compress(args) => commands::compress::run(args, cli.verbose, cli.quiet),
        Commands::Decompress(args) => commands::decompress::run(args, cli.verbose, cli.quiet),
        Commands::Info(args) => commands::info::run(args, cli.verbose, cli.quiet),
        Commands::Batch(args) => commands::batch::run(args, cli.verbose, cli.quiet),
    }
}