use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Takes the source image and makes an image suitable for Instagram, aka 4/5 aspect ratio.
pub struct InstasizeArgs {
    /// Source image
    pub source: String,
    /// Target image 
    pub target: String
}
