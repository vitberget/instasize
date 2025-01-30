use anyhow::bail;
use clap::{command, Parser, ValueHint};
use image::{Pixel, Rgba};
use regex::Regex;

/// Takes the source image and makes an image suitable for Instagram, aka 4/5 aspect ratio.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct InstasizeArgs {
    /// Source image or directory with source images
    #[arg(value_hint = ValueHint::AnyPath)]
    pub source: String,

    /// Target image or directory
    #[arg(value_hint = ValueHint::AnyPath)]
    pub target: String,

    /// Make the background border a blurred version of the image
    #[arg(long, short, conflicts_with = "color")]
    pub blur: Option<f32>,

    /// Make the blurred background darker
    #[arg(long, short, requires = "blur", allow_hyphen_values = true)]
    pub adjust_brightness: Option<i32>,

    /// Use this color for the background
    #[arg(long, short, conflicts_with = "blur", value_parser = parse_color)]
    pub color: Option<Rgba<u8>>
}

fn parse_color(text: &str) -> anyhow::Result<Rgba<u8>> {
    let reg = Regex::new("#[0-9a-f]{6}")?;

    match text.to_lowercase().as_str() {
        "black" => Ok(*Rgba::from_slice(&[0, 0, 0, 0])),
        "white" => Ok(*Rgba::from_slice(&[0xFF, 0xFF, 0xFF, 0])),

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
