use std::fs::File;
use std::error::Error;
use std::str::FromStr;
use std::fmt::Formatter;
use std::fmt::Display;
use csv;
use clap::Clap;

use std::path::PathBuf;

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
impl Display for AsciiCharacterParseError {
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
impl Display for TerminatorParseError {
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

#[derive(Debug)]
struct CSVColumn {
    header: String,
    values: Vec<String>,
    // type?
}

impl CSVColumn {
    pub fn new() -> Self {
        CSVColumn { header: String::default(), values: Vec::new() }
    }
    pub fn from_header(header: String) -> Self {
        CSVColumn { header, values: Vec::new() }
    }
    fn set_value(&mut self, index: usize, value: String) {
        while index >= self.values.len() {
            self.values.push(String::default());            
        } 
        self.values[index] = value;
    }
    fn get_value(&mut self, index: usize) -> Option<&String> {
        self.values.get(index)
    }
}

#[derive(Debug)]
struct CSV {
    columns: Vec<CSVColumn>,
}

impl CSV {
    pub fn new() -> Self {
        CSV { columns: Vec::new() }
    }
    // fn get_or_create_column(column_index header: String) {
    //     if 
    // }

    fn new_column(&mut self, header: String) {        
        self.columns.push(CSVColumn::from_header(header));
    }

    fn get_column(&mut self, column_index: usize) -> &mut CSVColumn {
        while column_index >= self.columns.len() {
            self.columns.push(CSVColumn::new());            
        } 
        return &mut self.columns[column_index]
    }
}

impl<R> From<csv::Reader<R>> for CSV where R: std::io::Read {
    fn from(mut reader: csv::Reader<R>) -> Self { 

        let mut csv = CSV::new();

        if reader.has_headers() {
            let headers = reader.headers()
                .expect("Error reading CSV file");      
            for header in headers {
                csv.new_column(header.to_owned());
            }
        }
            
        for (row_index, row) in reader.records().into_iter().enumerate() {
            let row = row.expect(&format!("Error reading row {} in CSV file", row_index));
            for (column_index, value) in row.into_iter().enumerate() {                
                csv.get_column(column_index).set_value(row_index, value.to_owned());
            }
        }

        csv
    }
}

fn main() {
    let options = Options::parse();
    let csv = CSV::from(options.build_reader());

    println!("{:?}", csv);

    // println!("{:?}", reader.headers());
    // for row in reader.records() {
    //     println!("{:?}", row);
    // }
    
}

