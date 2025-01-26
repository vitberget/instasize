use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Takes the source images and makes a new image where they are stacked vertically.
pub struct InstastackArgs {
    #[structopt(required = true)]
    /// The images you want to stack
    pub source: Vec<String>,
    /// The resulting image
    pub target: String
}
