use std::fmt::Display;
use std::str::FromStr;

use clap::ValueEnum;
use current_locale::*;

use crate::utils::errors::*;

/**
 * Convenience function that grabs the locale from the OS, if no locale was 
 * set. If the locale cannot be determined, it defaults to en_US.UTF-8. In that
 * case it also prints a warning.
 */
fn default_locale() -> String {
    let maybe_locale = current_locale();
    let locale = maybe_locale.unwrap_or_else(|e| {
        log::warn!("Cannot determine system locale: {}", e);
        "en_US.UTF-8".to_owned()
    });
    locale
}

#[derive(clap::Args, Clone, Debug)]
/// CSV-specific configuration options
pub struct Csv {
    #[clap(long)]
    /// Do not treat the first row as a header
    pub no_headers: bool,

    #[clap(short, long, value_enum, default_value(","))]
    /// CSV column delimiter character
    pub column_delimiter: Ascii,

    #[clap(short, long, default_value(Terminator::default()))]
    /// CSV row terminator sequence
    pub row_teminator: Terminator,

    #[clap(long, value_enum, default_value("all"))]
    /// Trim whitespace from the beginning and end of CSV cells
    pub trim: Trim,

    #[clap(long, default_value("\""),  value_parser = Ascii::from_str, num_args = 0..)]
    /// Interpret character as an escape inside quoted CSV cells; use with no
    /// argument means to for no escapes
    pub escape: Option<Ascii>,

    #[clap(long, default_value("#"),  value_parser = Ascii::from_str, num_args = 0..)]
    /// Interpret character as the start of a comment (until end of line); use with no
    /// argument means to for no comments
    pub comment: Option<Ascii>,

    #[clap(long, default_value(default_locale()))]
    /// Interpret input file using this locale
    pub locale: String,

    // #[clap(long)]
    // pub each_row_same_length: bool, // flexible length records by default by default

    // #[clap(long)]
    // /// Do not treat ' specially
    // pub ignore_quotes: bool,

    // #[clap(long)]
    // /// Do not treat " specially
    // pub ignore_double_quotes: bool, 

    // #[clap(long)]
    // /// Interpret # as a start of an end-of-line comment
    // pub hash_comments: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, ValueEnum)]
pub enum Trim {
    /// Preserve whitespace in all cells
    None, 
    /// Trim whitespace in headers, but not in other cells,
    OnlyHeaders,
    /// Trim whitespace in all cells, except for headers,
    ExceptHeaders,
    /// Trim whitespace in all cells
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Ascii {
    pub character: char,
    pub byte: u8,
}

impl From<Ascii> for u8 {
    fn from(ascii: Ascii) -> Self {
        ascii.byte
    }
}

impl From<Ascii> for char {
    fn from(ascii: Ascii) -> Self {
        ascii.character
    }
}

impl Ascii {
    pub fn as_byte(&self) -> u8 {
        self.byte
    }
    pub fn as_char(&self) -> char {
        self.character
    }
}

impl FromStr for Ascii {
    type Err = CsvConfigError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        { string.is_ascii() }.or_fail_with(|| CsvConfigError::expected_ascii(string))?;
        { string.len() == 1 }.or_fail_with(|| CsvConfigError::expected_ascii(string))?;

        let byte = string.as_bytes()[0usize];        
        let character = char::from_u32(byte as u32).unwrap();

        Ok(Ascii{ 
            character,
            byte,            
        })
    }
}

// impl Ascii {
//     pub fn optinal_from_str(string: &str) -> Result<Self, <Ascii as FromStr>::Err> {
//         if 0 == string.len() {
//             None
//         } else {
//             Some(Self::from_str(string))
//         }
//     }
// }

impl Ascii {
    pub fn optional_from_str(string: &str) -> Result<Option<Self>, <Ascii as FromStr>::Err> {
        if 0 == string.len() {
            Ok(None)
        } else {
            Self::from_str(string).map(Some)
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum Terminator {
    // Carriage return and new line
    CRLF,
    // Only carriage return character
    CR,
    // Only line feed character
    LF,
}

impl Display for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminator::CRLF => f.write_str("CRLF"),
            Terminator::CR => f.write_str("CR"),
            Terminator::LF => f.write_str("LF"),
        }
    }
}

impl Default for Terminator {
    fn default() -> Self {
        Self::CRLF
    }
}

impl clap::ValueEnum for Terminator {
    fn value_variants<'a>() -> &'a [Self] {
        &[Terminator::CRLF, Terminator::CR, Terminator::LF]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let possible_value = match self {
            Terminator::CRLF => {
                clap::builder::PossibleValue::new("CRLF")
                .help("Carriage return and new line (\\r\\n)")
            }
            Terminator::CR => {
                clap::builder::PossibleValue::new("CR")
                .help("Only carriage return character (\\r)")
            }
            Terminator::LF => {
                clap::builder::PossibleValue::new("LF")
                .help("Only line feed character (\\n)")
            }
        };

        Some(possible_value)
    }
}

impl Terminator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Terminator::CRLF => "CRLF",
            Terminator::CR => "CR",
            Terminator::LF => "LF",
        }
    }
}

impl AsRef<str> for Terminator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Terminator> for clap::builder::OsStr {
    fn from(value: Terminator) -> Self {
        clap::builder::OsStr::from(value.as_str())
    }
}

#[derive(Debug)]
pub enum CsvConfigError {
    ExpectedASCII { string: String },
    ExpectedCharacter { string: String },
    ExpectedLineTerminator { string: String }
}

impl std::error::Error for CsvConfigError {}

impl CsvConfigError {
    pub fn expected_ascii(string: impl Into<String>) -> Self {
        CsvConfigError::ExpectedASCII { string: string.into() }
    }
    pub fn expected_character(string: impl Into<String>) -> Self {
        CsvConfigError::ExpectedCharacter { string: string.into() }
    }
    pub fn expected_line_terminator(string: impl Into<String>) -> Self {
        CsvConfigError::ExpectedLineTerminator { string: string.into() }
    }
}

impl Display for CsvConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CsvConfigError::ExpectedASCII { string } => write!(f, "Expected an ASCII string but found \"{string}\"."),
            CsvConfigError::ExpectedCharacter { string } => write!(f, "Expected a single character but found string \"{string}\" of length {}.", string.chars().count()),
            CsvConfigError::ExpectedLineTerminator { string } => write!(f, "Expected a line terminator: \"CRLF\", \"CR\", \"LF\", \"\\r\\n\" or any single character, but found string \"{string}\""),
        }
    }
}
