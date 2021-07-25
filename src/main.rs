use std::fs::File;
use std::str::FromStr;
use std::fmt::Formatter;
use std::fmt::Display;
use std::path::PathBuf;

use csv;
use ncurses;

use clap::Clap;

use hamstercsv::csv::*;
use unicode_segmentation::UnicodeSegmentation;

use log;


// TODO add `about(...)`s
// TODO maybe stdin support

#[derive(Clap, Debug)]
struct Options {
    #[clap(long)]
    no_headers: bool,

    #[clap(short, long, default_value(","), about(""))]
    column_delimiter: AsciiCharacter,

    #[clap(short, long, default_value("CRLF"))]
    row_teminator: Terminator,

    #[clap(short, long)]
    escape: Option<AsciiCharacter>,

    #[clap(long)]
    comment: Option<AsciiCharacter>,

    #[clap(long, default_value("all"))]
    trim_whitespace: Trim,

    #[clap(long, default_value("\""))]
    quote: AsciiCharacter,

    #[clap(long)]
    ignore_quotes: bool,

    #[clap(long)]
    ignore_double_quotes: bool, 

    #[clap(long)]
    each_row_same_length: bool, // flexible length records by default by default

    #[clap(long, default_value("en_US.UTF-8"))]
    locale: String,

    path: PathBuf,
}

#[derive(Debug)]
struct Trim(csv::Trim);
impl Trim {
    pub fn as_csv_trim(&self) -> csv::Trim {
        self.0
    }
}
impl FromStr for Trim {    
    type Err = TrimParseError;
    fn from_str(string: &str) -> Result<Self, Self::Err> { 
        match string.to_lowercase().as_str() {
            "none" => Ok(Trim(csv::Trim::None)),
            "headers" => Ok(Trim(csv::Trim::Headers)),
            "fields" => Ok(Trim(csv::Trim::Fields)),
            "all" => Ok(Trim(csv::Trim::All)),
            other => Err(TrimParseError(other.to_owned()))
        }
    }
}

#[derive(Debug)]
struct TrimParseError(String);
impl Display for TrimParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
        write!(f, "Invalid trim value \"{}\": expected \"none\", \"headers\", \"fields\", or \"all\".", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AsciiCharacter(u8);
impl AsciiCharacter {
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
impl FromStr for AsciiCharacter {    
    type Err = AsciiCharacterParseError;
    fn from_str(string: &str) -> Result<Self, Self::Err> { 
        if string.len() != 1 {
            return Err(AsciiCharacterParseError(string.to_owned()))
        } 
        Ok(AsciiCharacter(string.as_bytes()[0]))
    }
}
impl Into<u8> for AsciiCharacter {
    fn into(self) -> u8 { self.0 }
}
impl Into<u8> for &AsciiCharacter {
    fn into(self) -> u8 { self.0.clone() }
}

#[derive(Debug)]
struct AsciiCharacterParseError(String);
impl std::fmt::Display for AsciiCharacterParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
        write!(f, "Invalid character value \"{}\": expected a single character. Multi-byte characters are not supported.", self.0)
    }
}

#[derive(Debug)]
struct Terminator(csv::Terminator);
impl Terminator {
    fn as_csv_terminator(&self) -> csv::Terminator {
        self.0
    }
}
impl FromStr for Terminator {    
    type Err = TerminatorParseError;    
    fn from_str(string: &str) -> Result<Self, Self::Err> { 
        match string {
            "CRLF" | "crlf" | "default" => Ok(Terminator(csv::Terminator::CRLF)),
            string if string.as_bytes().len() == 1 => Ok(Terminator(csv::Terminator::Any(string.as_bytes()[0]))),
            string => Err(TerminatorParseError(string.to_owned()))
        }
    }
}
impl Into<csv::Terminator> for Terminator {
    fn into(self) -> csv::Terminator { self.0 }
}

#[derive(Debug)]
struct TerminatorParseError(String);
impl std::fmt::Display for TerminatorParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
        write!(f, "Invalid terminator \"{}\": terminators must be a single character or \"CRLF\", \"crlf\", or \"default\".", self.0)
    }
}

impl Options {
    pub fn build_reader(&self) -> csv::Reader<File> {
        let mut builder = csv::ReaderBuilder::new();

        builder
            .has_headers(!self.no_headers)
            .delimiter(self.column_delimiter.as_u8())
            .terminator(self.row_teminator.as_csv_terminator())
            .escape(self.escape.as_ref().map(|c| c.as_u8()))
            .comment(self.comment.as_ref().map(|c| c.as_u8()))
            .quote(self.quote.as_u8())
            .quoting(!self.ignore_quotes)                        
            .double_quote(!self.ignore_double_quotes)
            .trim(self.trim_whitespace.as_csv_trim())
            .flexible(!self.each_row_same_length);

        builder
            .from_path(&self.path)
            .expect(&format!("Cannot open CSV file {:?}.", self.path))
    }
}

enum ColorScheme {

}

static COLOR_FOREGROUND: i16 = 24;
static COLOR_BACKGROUND: i16 = 25;

static COLOR_VALUES_BACKGROUND_EVEN: i16 = 16;
static COLOR_VALUES_FOREGROUND_EVEN: i16 = 17;

static COLOR_HEADER_FOREGROUND_EVEN: i16 = 18;
static COLOR_HEADER_BACKGROUND_EVEN: i16 = 19;

static COLOR_VALUES_BACKGROUND_ODD: i16 = 20;
static COLOR_VALUES_FOREGROUND_ODD: i16 = 21;

static COLOR_HEADER_FOREGROUND_ODD: i16 = 22;
static COLOR_HEADER_BACKGROUND_ODD: i16 = 23;

static COLOR_PAIR: i16 = 1;
static COLOR_VALUES_PAIR_EVEN: i16 = 2;
static COLOR_HEADER_PAIR_EVEN: i16 = 3;
static COLOR_VALUES_PAIR_ODD: i16 = 4;
static COLOR_HEADER_PAIR_ODD: i16 = 5;

struct CSVDisplay {
    column: usize,
    row: usize,

    column_width: usize, // Invariant: > 0 & <= screen_width
    row_height: usize,   // Invariant: > 0 & <= screen_height - 2

    screen_height: usize, 
    screen_width: usize,
    
    csv: CSVFile,    
}
impl CSVDisplay {
    pub fn from(csv: CSVFile, options: &Options) -> Self {

        ncurses::setlocale(ncurses::LcCategory::all, options.locale.as_str()); // TODO is this actually configurable to any reasonable extent?

        ncurses::initscr();
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();

        ncurses::start_color();

        ncurses::init_color(COLOR_FOREGROUND, 100 * 4, 100 * 4, 100 * 4);
        ncurses::init_color(COLOR_BACKGROUND, 100 * 4, 100 * 4, 100 * 4);
        ncurses::init_pair (COLOR_PAIR, COLOR_FOREGROUND, COLOR_BACKGROUND);

        ncurses::init_color(COLOR_HEADER_FOREGROUND_EVEN, 0, 43 * 4, 54 * 4);
        ncurses::init_color(COLOR_HEADER_BACKGROUND_EVEN, 142 * 4, 161 * 4, 161 * 4);    
        ncurses::init_pair (COLOR_HEADER_PAIR_EVEN, COLOR_HEADER_FOREGROUND_EVEN, COLOR_HEADER_BACKGROUND_EVEN);

        ncurses::init_color(COLOR_VALUES_BACKGROUND_EVEN, 0, 43 * 4, 54 * 4);
        ncurses::init_color(COLOR_VALUES_FOREGROUND_EVEN, 55 * 4, 109 * 4, 114 * 4);
        ncurses::init_pair (COLOR_VALUES_PAIR_EVEN, COLOR_VALUES_FOREGROUND_EVEN, COLOR_VALUES_BACKGROUND_EVEN);  

        ncurses::init_color(COLOR_HEADER_FOREGROUND_ODD, 0, 43 * 4, 54 * 4);
        ncurses::init_color(COLOR_HEADER_BACKGROUND_ODD, 142 * 4, 161 * 4, 161 * 4);
        ncurses::init_pair (COLOR_HEADER_PAIR_ODD, COLOR_HEADER_FOREGROUND_ODD, COLOR_HEADER_BACKGROUND_ODD);

        ncurses::init_color(COLOR_VALUES_BACKGROUND_ODD, 0, 43 * 4, 54 * 4);
        ncurses::init_color(COLOR_VALUES_FOREGROUND_ODD, 142 * 4, 161 * 4, 161 * 4);    
        ncurses::init_pair (COLOR_VALUES_PAIR_ODD, COLOR_VALUES_FOREGROUND_ODD, COLOR_VALUES_BACKGROUND_ODD);
        
        ncurses::bkgd(' ' as ncurses::chtype | ncurses::COLOR_PAIR(COLOR_PAIR) as ncurses::chtype);

        let mut screen_height: i32 = 0;
        let mut screen_width: i32 = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut screen_height, &mut screen_width);

        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        ncurses::clear();

        CSVDisplay { 
            csv, 

            row: 0, 
            column: 0, 

            column_width: 10, 
            row_height: 1, 

            screen_height: screen_height as usize, 
            screen_width: screen_width as usize,
        }
    }

    pub fn run(&mut self) {

        let how_many_columns_visible = self.screen_width / self.column_width;
        let how_many_rows_visible = self.screen_height / self.row_height - 1 /* headers */ - 1 /* status bar */; 

        let first_column = self.column;
        let last_column = self.column + how_many_columns_visible;

        let first_row = self.row;
        let last_row = std::cmp::min(self.row + how_many_rows_visible, self.csv.column_count());

        println!("{}", first_row);
        println!("{}", last_row);

        let cell_dimensions = CellDimentions { width: self.column_width - 1, height: 1 };

        //let empty = CSVItem::default();
        for column_index in first_column..last_column {

            if let Some(column) = self.csv.get_column(column_index) {

                ncurses::attron(ncurses::A_BOLD());                
                ncurses::attron(ncurses::COLOR_PAIR(if column_index % 2 == 0 { COLOR_HEADER_PAIR_EVEN } else { COLOR_HEADER_PAIR_ODD }));
                
                ncurses::mv(0, (column_index * self.column_width) as i32);
                
                ncurses::addstr(column.header().to_owned().cut_or_pad_to(cell_dimensions.width, " ").join("").as_str());
                ncurses::addstr(" ");
                
                ncurses::attroff(ncurses::A_BOLD());                
                ncurses::attroff(ncurses::COLOR_PAIR(if column_index % 2 == 0 { COLOR_HEADER_PAIR_EVEN } else { COLOR_HEADER_PAIR_ODD }));
                ncurses::attron(ncurses::COLOR_PAIR(if column_index % 2 == 0 { COLOR_VALUES_PAIR_EVEN } else { COLOR_VALUES_PAIR_ODD }));
                
                for line in 1..(self.screen_height - 1) {
                    let rows = column.value(first_row + line - 1)
                        .map_or_else(Vec::new, |item| item.cut_or_pad_to(cell_dimensions, " "));
                    ncurses::mv(line as i32, (column_index * self.column_width) as i32);
                    for row in rows {
                        ncurses::addstr(row.join("").as_str());
                        ncurses::addstr(" ");
                        //ncurses::addstr("value");
                    }                    
                }
                ncurses::attroff(ncurses::COLOR_PAIR(if column_index % 2 == 0 { COLOR_VALUES_PAIR_EVEN } else { COLOR_VALUES_PAIR_ODD }));
            }
        }

        ncurses::mv(self.screen_height as i32 - 1, 0);
        ncurses::addstr(&format!("row: {}-{}, cols: {}-{}", first_row, last_row, first_column, last_column));

        while ncurses::getch() != 'q' as i32 { }
    }
}

impl Drop for CSVDisplay {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

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

