use clap::{command, Parser, ValueHint};

/// Takes the source image and makes an image suitable for Instagram, aka 4/5 aspect ratio.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct InstawidthArgs {
    /// Source image or directory with source images
    #[arg(value_hint = ValueHint::AnyPath)]
    pub source: String,

    /// Target image or directory
    #[arg(value_hint = ValueHint::AnyPath)]
    pub target: String,
}
