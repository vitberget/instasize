use std::path::Path;

use anyhow::ensure;
use clap::Parser;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageReader, Pixel, Primitive};

#[derive(Debug, Parser)]
pub struct InstastackArgs {
    #[structopt(required = true)]
    pub sources: Vec<String>,
    pub target: String
}

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

fn get_image_width(images: &[DynamicImage]) -> anyhow::Result<u32> {
    let widths: Vec<u32> = images.iter()
        .map(|img| img.dimensions())
        .map(|dim| dim.0)
        .collect();
    get_width(&widths)
}

fn get_width(widths: &[u32]) -> anyhow::Result<u32> {
    let (same_width, width) = widths.iter().fold((true, 0), |(acc, width), w| {
        (acc && (width == 0 || width == *w), *w)
    });
    ensure!(same_width, "Images not of same width");
    Ok(width)
}

fn get_total_height(images: &[DynamicImage]) -> u32 {
    images.iter()
        .map(|img| img.dimensions())
        .map(|dim| dim.1)
        .sum()
}

fn stack_them<P,S>(source_images: &Vec<ImageBuffer<P, Vec<S>>>, width: u32, height: u32) -> anyhow::Result<ImageBuffer<P, Vec<S>>>
where
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static, 
{
    let mut target_image = image::ImageBuffer::new(width, height);
    let mut current_y: u32 = 0;

    for image in source_images {
        target_image.copy_from(image, 0, current_y)?;
        current_y += image.dimensions().1;
    }

    Ok(target_image)
}


fn get_source_images(args: &InstastackArgs) -> anyhow::Result<Vec<DynamicImage>> {
    let mut images: Vec<DynamicImage> = vec![];

    for filename in args.sources.clone() {
        let path = Path::new(&filename);
        ensure!(path.exists(), "Source {filename} does not exist");
        let image = ImageReader::open(path)?.decode()?;
        images.push(image);
    }

    Ok(images)
}

fn get_target_path(filename: &str) -> anyhow::Result<&Path> {
    let source = Path::new(filename);
    ensure!(!source.exists(), "Target {filename} already exist");
    Ok(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_same_witdh() -> anyhow::Result<()> {
        let widths: Vec<u32> = vec![123,123,123]; 
        assert_eq!(get_width(&widths)?, 123);
        Ok(())
    }

    #[test]
    fn test_not_same_widths() {
        let widths: Vec<u32> = vec![1,2,3];
        let ensure = get_width(&widths);
        assert!(ensure.is_err())
    }
}
