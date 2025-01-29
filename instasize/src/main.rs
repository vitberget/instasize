use clap::Parser as _;

use crate::args::InstasizeArgs;
use crate::directory::insta_directory;
use crate::one_image::insta_one_file;
use crate::paths::{get_source_path, get_target_path};

mod args;
mod one_image;
mod directory;
mod paths;

fn main() -> anyhow::Result<()>{
    let args = InstasizeArgs::parse(); 

    let source = get_source_path(&args)?;
    let target = get_target_path(&args)?;

    if source.is_file() {
        insta_one_file(source, target, &args)?;
    } else {
        insta_directory(source, target, &args)?;
    }

    Ok(())
}
