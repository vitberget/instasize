use image::{GenericImage as _, ImageBuffer, Pixel, Primitive};

pub fn stack_them<P,S>(source_images: &Vec<ImageBuffer<P, Vec<S>>>, width: u32, height: u32) -> anyhow::Result<ImageBuffer<P, Vec<S>>>
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
