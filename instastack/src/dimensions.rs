use anyhow::ensure;
use image::{DynamicImage, GenericImageView as _};

pub fn get_image_width(images: &[DynamicImage]) -> anyhow::Result<u32> {
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

pub fn get_total_height(images: &[DynamicImage]) -> u32 {
    images.iter()
        .map(|img| img.dimensions())
        .map(|dim| dim.1)
        .sum()
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
