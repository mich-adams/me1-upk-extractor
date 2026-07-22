use anyhow::Result;
use clap::Parser;
use log::info;
use std::path::PathBuf;

mod upk;
mod extractor;
mod asset_types;

use extractor::AssetExtractor;

#[derive(Parser, Debug)]
#[command(name = "ME1 UPK Extractor")]
#[command(about = "Extract models, animations, and textures from Mass Effect 1 UPK files", long_about = None)]
struct Args {
    /// Input directory containing UPK files
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for extracted assets
    #[arg(short, long)]
    output: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Extract only specific asset types (models, animations, textures)
    /// If not specified, extracts all types
    #[arg(short, long)]
    asset_types: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_default_env()
        .filter_level(log_level.parse()?)
        .init();

    info!("ME1 UPK Asset Extractor starting");
    info!("Input directory: {}", args.input.display());
    info!("Output directory: {}", args.output.display());

    // Validate input directory
    if !args.input.exists() {
        return Err(anyhow::anyhow!("Input directory does not exist: {}", args.input.display()));
    }

    if !args.input.is_dir() {
        return Err(anyhow::anyhow!("Input path is not a directory: {}", args.input.display()));
    }

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output)?;

    // Initialize extractor
    let extractor = AssetExtractor::new(args.output);

    // Process all UPK files in directory tree
    extractor.process_directory(&args.input)?;

    info!("Extraction complete!");
    Ok(())
}
