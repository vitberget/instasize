use clap::Parser as _;

use crate::args::InstawidthArgs;
use crate::process_image::{insta_directory, insta_one_file};
use crate::paths::{get_source_path, get_target_path};

mod args;
mod paths;
mod process_image;

fn main() -> anyhow::Result<()> {
    let args = InstawidthArgs::parse(); 

    let source = get_source_path(&args)?;
    let target = get_target_path(&args)?;

    if source.is_dir() {
        insta_directory(source, target)?;
    } else {
        insta_one_file(source, target)?;
    }

    Ok(())
}
