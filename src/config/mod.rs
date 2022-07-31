pub mod csv;
pub use self::csv::*;

use std::path::PathBuf;

pub use clap::Parser;

#[derive(Parser)]
#[clap(author("Konrad Siek <konrad.siek@gmail.com>"))]
#[clap(version)]
#[clap(about("CSV viewer for large files"))]
pub struct Config {
    #[clap(flatten)]
    csv: CSV,

    #[clap()]
    /// Path to input file
    pub path: PathBuf,
}

