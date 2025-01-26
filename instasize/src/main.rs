use std::path::Path;

use anyhow::ensure;
use clap::Parser;
use directory::insta_directory;
use one_image::insta_one_file;

mod one_image;
mod directory;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Takes the source image and makes an image suitable for Instagram, aka 4/5 aspect ratio.
pub struct InstasizeArgs {
    /// Source image
    pub source: String,
    /// Target image 
    pub target: String
}

fn main() -> anyhow::Result<()>{
    let args = InstasizeArgs::parse(); 

    let source = get_source_path(&args)?;
    let target = get_target_path(&args)?;

    if source.is_file() {
        insta_one_file(source, target)?;
    } else {
        insta_directory(source, target)?;
    }

    Ok(())
}

fn get_source_path(args: &InstasizeArgs) -> anyhow::Result<&Path> {
    let source = Path::new(&args.source);
    ensure!(source.exists(), "Source {} does not exist", args.source);
    Ok(source)
}

fn get_target_path(args: &InstasizeArgs) -> anyhow::Result<&Path> {
    let target = Path::new(&args.target);
    Ok(target)
}
