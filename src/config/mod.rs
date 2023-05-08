pub mod csv;
pub use self::csv::*;

pub mod aesthetics;
pub use self::aesthetics::*;

use std::path::PathBuf;

pub use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author("Konrad Siek <konrad.siek@gmail.com>"))]
#[clap(version)]
#[clap(about("CSV viewer for large files"))]
pub struct Config {
    #[clap(flatten)]
    /// CSV-specific options
    pub csv: Csv,

    #[clap(flatten)]
    /// Display options
    pub aesthetics: Aesthetics,

    #[clap()]
    /// Path to input CSV file
    pub path: PathBuf,
}

impl Config {
    pub fn path_str(&self) -> &str {
        self.path.as_os_str().to_str().unwrap()
    }
}