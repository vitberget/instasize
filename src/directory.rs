use std::path::Path;

use anyhow::ensure;

use crate::one_image::insta_one_file;

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
