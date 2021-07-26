use std::fs::File;
use std::str::FromStr;
use std::fmt::Formatter;
use std::fmt::Display;
use std::path::PathBuf;

use csv;
use ncurses;

use clap::Clap;

use hamstercsv::csv::*;
use hamstercsv::screen::*;
use hamstercsv::cli::*;
use unicode_segmentation::UnicodeSegmentation;

use log;


// TODO add `about(...)`s
// TODO maybe stdin support


fn main() {
    simple_logging::log_to_file("hamstercsv.log", log::LevelFilter::Info);

    let options = Options::parse();
    println!("...0");
    let csv = CSVFile::from(options.build_reader());
    println!("...1");
    let mut display = CSVDisplay::from(csv, &options);    
    println!("...2");
    display.run();

    // let column = csv.get_column(0).unwrap();

    // for value in column.values() {
    //     //ncurses::attron(ncurses::COLOR_PAIR(COLOR_VALUES_PAIR));
    //     ncurses::addstr(value);
    //     ncurses::mv()
    //     //ncurses::attroff(ncurses::COLOR_PAIR(COLOR_VALUES_PAIR));
    // }

    // //ncurses::attron(ncurses::A_BOLD());
    // ncurses::addstr("<-Press Space->");-+
    // while ncurses::getch() != ' ' as i32
    // { }
    //ncurses::attroff(ncurses::A_BOLD());
    
    //println!("{:?}", csv);

    // println!("{:?}", reader.headers());
    // for row in reader.records() {
    //     println!("{:?}", row);
    // }

    // End procedure:
    //ncurses::mv(self.screen_height - 1, 0);
    //ncurses::prompt();
}

