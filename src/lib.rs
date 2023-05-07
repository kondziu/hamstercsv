// pub mod screen;
pub mod config;
pub mod data;
pub mod utils;


use std::convert::TryFrom;

use clap::Parser;
use config::Config;
use data::Csv;

pub struct HamsterCsv;

impl HamsterCsv {

    pub fn read_config() -> Config {
        Config::parse()
    }

    // pub fn read_csv(config: &Config) -> data::Csv {
    //     let reader: csv::ReaderBuilder = config.into();
    //     let builder = CsvBuilder::try_from(&reader);
    //     todo!()
    // }
}