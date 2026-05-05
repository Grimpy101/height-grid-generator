use std::path::PathBuf;

/// A simple program that creates a height grid
/// from a point cloud
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Filepath to the point cloud
    #[arg(short, long)]
    pub input_filepath: PathBuf,

    /// Output filepath
    #[arg(short, long)]
    pub output_filepath: PathBuf,

    /// Grid resolution (in meters)
    #[arg(short, long)]
    pub resolution: f32,

    /// Determines whether the file should be binary or textual
    #[arg(short, long, default_value_t = false)]
    pub binary: bool,

    /// Format to write to (ply or custom)
    #[arg(short, long)]
    pub format: String,
}
