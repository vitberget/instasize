use clap::Parser as _;
use dimensions::{get_image_width, get_total_height};

use crate::args::InstastackArgs;
use crate::input::{get_source_images, get_target_path};
use crate::output::stack_them;

mod args;
mod dimensions;
mod input;
mod output;

fn main() -> anyhow::Result<()> {
    let args = InstastackArgs::parse();
    let images = get_source_images(&args)?;
    let target_path = get_target_path(&args.target)?;

    let width = get_image_width(&images)?;
    let total_height = get_total_height(&images);
   
    let images = images.iter()
        .flat_map(|img| img.as_rgb8())
        .map(|img| img.to_owned())
        .collect();

    let target_image = stack_them(&images, width, total_height)?;
    
    target_image.save(target_path)?;
    Ok(())
}
