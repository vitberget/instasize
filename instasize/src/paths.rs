use std::path::Path;

use anyhow::ensure;

use crate::InstasizeArgs;

pub fn get_source_path(args: &InstasizeArgs) -> anyhow::Result<&Path> {
    let source = Path::new(&args.source);
    ensure!(source.exists(), "Source {} does not exist", args.source);
    Ok(source)
}

pub fn get_target_path(args: &InstasizeArgs) -> anyhow::Result<&Path> {
    let target = Path::new(&args.target);
    Ok(target)
}
