use clap::Clap;

use hamstercsv::csv::*;
use hamstercsv::screen::*;
use hamstercsv::cli::*;

use log;


// TODO add `about(...)`s
// TODO maybe stdin support


fn main() {
    simple_logging::log_to_file("hamstercsv.log", log::LevelFilter::Info).unwrap();

    let options = Options::parse();
    let csv = CSVFile::from(options.build_reader());
    let mut display = CSVDisplay::from(csv, &options);    
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

