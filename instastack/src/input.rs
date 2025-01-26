use std::path::Path;

use anyhow::ensure;
use image::{DynamicImage, ImageReader};

use crate::args::InstastackArgs;

pub fn get_source_images(args: &InstastackArgs) -> anyhow::Result<Vec<DynamicImage>> {
    let mut images: Vec<DynamicImage> = vec![];

    for filename in args.source.clone() {
        let path = Path::new(&filename);
        ensure!(path.exists(), "Source {filename} does not exist");
        let image = ImageReader::open(path)?.decode()?;
        images.push(image);
    }

    Ok(images)
}

pub fn get_target_path(filename: &str) -> anyhow::Result<&Path> {
    let source = Path::new(filename);
    ensure!(!source.exists(), "Target {filename} already exist");
    Ok(source)
}
