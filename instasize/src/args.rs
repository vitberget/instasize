use anyhow::bail;
use clap::{command, Parser};
use image::{Pixel, Rgba};
use regex::Regex;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Takes the source image and makes an image suitable for Instagram, aka 4/5 aspect ratio.
pub struct InstasizeArgs {
    /// Source image
    pub source: String,
    /// Target image 
    pub target: String,

    #[clap(long, short, conflicts_with = "color")]
    /// Make the background border a blurred version of the image
    pub blur: Option<f32>,

    #[clap(long, short, requires = "blur", allow_hyphen_values = true)]
    /// Make the blurred background darker
    pub adjust_brightness: Option<i32>,

    #[clap(long, short, conflicts_with = "blur", value_parser = parse_color)]
    /// Use this color for the background
    pub color: Option<Rgba<u8>>
}


fn parse_color(text: &str) -> anyhow::Result<Rgba<u8>> {
    let reg = Regex::new("#[0-9a-f]{6}")?;

    match text.to_lowercase().as_str() {
        "black" => Ok(*Rgba::from_slice(&[0,0,0,0])),
        "white" => Ok(*Rgba::from_slice(&[0xFF,0xFF,0xFF,0])),

        text if reg.is_match(text) => parse_rgb(text),
        
        _ => bail!("I don't know what color {text:?} is")
    }
}

fn parse_rgb(text: &str) -> anyhow::Result<Rgba<u8>> {
    let mut num: u32 = u32::from_str_radix(&text[1..], 16)?;
    num <<= 8; 
    let bytes = num.to_be_bytes();

    Ok(*Rgba::from_slice(&bytes))
}
