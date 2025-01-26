use std::path::Path;

use anyhow::{bail, ensure, Context};
use image::{GenericImage, GenericImageView, ImageBuffer, ImageReader, Pixel, Primitive};

pub fn insta_one_file(source: &Path, target: &Path) -> anyhow::Result<()> {
    ensure!(!target.exists(), "Target {:?} already exist", target);
    println!("Processing {:?} -> {:?}", source, target);
    let source_image = ImageReader::open(source)?.decode()?;
    let target_image = extend_it(source_image.as_rgb8().context("fds")?)?;

    target_image.save(target)?;
    Ok(())
}

fn extend_it<I,P,S>(source_image: &I) -> anyhow::Result<ImageBuffer<P, Vec<S>>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static, 
{
    let (width, height) = source_image.dimensions();
    let new_width = (height * 4) / 5;
    let new_height = (width * 5) / 4;

    let mut target_image = image::ImageBuffer::new(u32::max(width, new_width), u32::max(height, new_height));
    if new_width > width {
        target_image.copy_from(source_image, (new_width - width) / 2, 0)?;
    } else if new_height > height {
        target_image.copy_from(source_image, 0, (new_height - height) / 2)?;
    } else {
        bail!("No resize?");
    }

    Ok(target_image)
}
