use clap::Parser;

/// Takes the source images and makes a new image where they are stacked vertically.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct InstastackArgs {
    /// The images you want to stack
    #[structopt(required = true)]
    #[arg(num_args = 2..)]
    pub source: Vec<String>,

    /// The resulting image
    pub target: String
}
