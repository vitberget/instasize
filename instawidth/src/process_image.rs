use std::path::Path;

use anyhow::ensure;
use image::ImageReader;
use image::imageops::FilterType;

pub fn insta_one_file(source: &Path, target: &Path) -> anyhow::Result<()> {
    const TARGET_WIDTH: u32 = 1080;

    ensure!(!target.exists(), "Target {target:?} already exist");
    println!("Processing {source:?} -> {target:?}");

    let source_image = ImageReader::open(source)?.decode()?;

    let original_width = source_image.width();
    let original_height = source_image.height();

    let target_height = (original_height * TARGET_WIDTH) / original_width;
    let target_image = source_image.resize(TARGET_WIDTH, target_height, FilterType::Lanczos3);

    target_image.save(target)?;

    Ok(())
}

pub fn insta_directory(source_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    ensure!(target_dir.is_dir(), "Source is a directory, target is not");

    for dir_entry in (source_dir.read_dir()?).flatten() {
        let source_path = dir_entry.path();
        let source = source_path.as_path();
        let mut target = target_dir.to_path_buf();
        target.push(dir_entry.file_name());

        if let Err(error) = insta_one_file(source, &target) {
            println!("  failed! {error}");
        }
    }

    println!("Finished.");

    Ok(())
}
