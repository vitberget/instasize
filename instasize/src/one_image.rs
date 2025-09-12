use std::path::Path;

use anyhow::ensure;
use image::{DynamicImage, ImageReader};
use image::imageops::{self, FilterType};
use image::{GenericImage, Rgb, Rgba, RgbaImage};

use crate::args::InstasizeArgs;

pub fn insta_one_file(source: &Path, target: &Path, args: &InstasizeArgs) -> anyhow::Result<()> {
    ensure!(!target.exists(), "Target {target:?} already exist");
    println!("Processing {source:?} -> {target:?}");

    let source_image = ImageReader::open(source)?.decode()?;
    let target_image = extend_it(&source_image.into_rgba8(), args)?;
    let target_image = rgba8_to_rgb8(target_image);

    let dynamic: DynamicImage = target_image.into();
    let dynamic = dynamic.resize(1080, 1440, FilterType::Lanczos3);

    dynamic.save(target)?;

    Ok(())
}

fn extend_it(source_image: &RgbaImage, args: &InstasizeArgs) -> anyhow::Result<RgbaImage> {
    const ASPECT_WIDTH: u32 = 3;
    const ASPECT_HEIGHT: u32 = 4;

    let (width, height) = source_image.dimensions();

    let new_width = (height * ASPECT_WIDTH) / ASPECT_HEIGHT;
    let new_height = (width * ASPECT_HEIGHT) / ASPECT_WIDTH;

    let mut target_image = RgbaImage::new(u32::max(width, new_width), u32::max(height, new_height));

    if new_height != height || new_width != width {
        if let Some(color) = args.color { fill_target(&mut target_image, &color)?; }
        if let Some(blur_size) = args.blur { blur_target(source_image, &mut target_image, blur_size, args.adjust_brightness)?; }
    }

    if new_width > width {
        target_image.copy_from(source_image, (new_width - width) / 2, 0)?;
    } else if new_height > height {
        target_image.copy_from(source_image, 0, (new_height - height) / 2)?;
    } 

    Ok(target_image)
}

fn blur_target(
    source_image: &image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    target_image: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    blur_size: f32,
    adjust_brightness: Option<i32>
) -> anyhow::Result<()> {
    let blur_image = source_image.clone();
    let blur_image = imageops::resize(&blur_image, target_image.width(), target_image.height(), FilterType::Gaussian);
    let blur_image = imageops::blur(&blur_image, blur_size);
    let blur_image = if let Some(brightness) = adjust_brightness {
        imageops::brighten(&blur_image, brightness)
    } else {
        blur_image
    };
    target_image.copy_from(&blur_image, 0, 0)?;
    Ok(())
}

fn fill_target(image: &mut RgbaImage, color: &Rgba<u8>) -> anyhow::Result<()> {
    for y in 0..image.height() {
        for x in 0..image.width() {
            image.put_pixel(x, y, *color);
        }
    }
    Ok(())
}

fn rgba8_to_rgb8(input: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = input.width() as usize;
    let height = input.height() as usize;

    // Get the raw image data as a vector
    let input: &Vec<u8> = input.as_raw();

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width * height * 3];

    let mut i = 0;
    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for chunk in input.chunks(4) {
        // ... and copy each of them to output, leaving out the A byte
        output_data[i..i+3].copy_from_slice(&chunk[0..3]);
        i+=3;
    }

    // Construct a new image
    image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}
