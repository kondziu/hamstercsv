use std::fs::File;
use std::str::FromStr;
use std::fmt::Formatter;
use std::fmt::Display;
use std::path::PathBuf;

use csv;

use clap::Clap;

#[derive(Clap, Debug)]
pub struct Options {
    #[clap(long)]
    pub no_headers: bool,

    #[clap(short, long, default_value(","), about(""))]
    pub column_delimiter: AsciiCharacter,

    #[clap(short, long, default_value("CRLF"))]
    pub row_teminator: Terminator,

    #[clap(short, long)]
    pub escape: Option<AsciiCharacter>,

    #[clap(long)]
    pub comment: Option<AsciiCharacter>,

    #[clap(long, default_value("all"))]
    pub trim_whitespace: Trim,

    #[clap(long, default_value("\""))]
    pub quote: AsciiCharacter,

    #[clap(long)]
    pub ignore_quotes: bool,

    #[clap(long)]
    pub ignore_double_quotes: bool, 

    #[clap(long)]
    pub each_row_same_length: bool, // flexible length records by default by default

    #[clap(long, default_value("en_US.UTF-8"))]
    pub locale: String,

    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Trim(csv::Trim);
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
pub struct TrimParseError(String);
impl Display for TrimParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
        write!(f, "Invalid trim value \"{}\": expected \"none\", \"headers\", \"fields\", or \"all\".", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AsciiCharacter(u8);
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
pub struct AsciiCharacterParseError(String);
impl std::fmt::Display for AsciiCharacterParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> { 
        write!(f, "Invalid character value \"{}\": expected a single character. Multi-byte characters are not supported.", self.0)
    }
}

#[derive(Debug)]
pub struct Terminator(csv::Terminator);
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
pub struct TerminatorParseError(String);
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