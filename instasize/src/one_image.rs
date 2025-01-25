use std::path::Path;

use anyhow::{ensure, Context};
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
    if height > width {
        let new_width = (height * 4) / 5;

        let mut target_image = image::ImageBuffer::new(new_width, height);
        target_image.copy_from(source_image, (new_width - width) / 2, 0)?;

        Ok(target_image)
    } else {
        let mut target_image = image::ImageBuffer::new(width, height);
        target_image.copy_from(source_image, 0, 0)?;

        Ok(target_image)
    }
}
