use std::fmt::Display;
use std::str::FromStr;

use clap::clap_derive::ArgEnum;

use crate::utils::errors::*;

#[derive(clap::Args)]
pub struct CSV {
    // CSV format options
    #[clap(long)]
    pub no_headers: bool,

    #[clap(short, long, default_value(","))]
    pub column_delimiter: ASCII,

    #[clap(short, long, default_value("CRLF"))]
    pub row_teminator: Terminator,

    #[clap(long, value_enum, default_value("all"))]
    pub trim_whitespace: Trim,

    #[clap(short, long)]
    /// Interpret \ as an escape character inside quoted strings
    pub backslash_escape: bool,

    #[clap(long, default_value(""))] // en_US.UTF-8
    pub locale: String,

    // #[clap(long)]
    // pub each_row_same_length: bool, // flexible length records by default by default

    // #[clap(long)]
    // /// Do not treat `'` specially
    // pub ignore_quotes: bool,

    // #[clap(long)]
    // /// Do not treat `"` specially
    // pub ignore_double_quotes: bool, 

    #[clap(long)]
    /// Interpret # as a start of an end-of-line comment
    pub hash_comments: bool,
}

#[derive(Debug, ArgEnum, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Trim {None, Headers, Trim, All}

pub struct ASCII {
    pub character: char,
    pub byte: u8,
}

impl FromStr for ASCII {
    type Err = ConfigError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        check_or_else(string.is_ascii(), || ConfigError::expected_ascii(string))?;
        check_or_else(string.len() == 1, || ConfigError::expected_ascii(string))?;

        let byte = string.as_bytes()[0usize];        
        let character = char::from_u32(byte as u32).unwrap();

        Ok(ASCII{ 
            character,
            byte,            
        })
    }
}

pub struct Terminator(String);

impl Terminator {
    fn from_str_unchecked (string: &str) -> Self {
        Terminator(string.to_string())
    }
}

impl FromStr for Terminator {
    type Err = ConfigError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        check_or_else(string.is_ascii(), || ConfigError::expected_ascii(string))?;

        match string.to_uppercase().as_str() {
            // User-friendly variants
            "CRLF" => Ok(Terminator::from_str_unchecked("\r\n")),
            "CR" => Ok(Terminator::from_str_unchecked("\r")),
            "LF" => Ok(Terminator::from_str_unchecked("\n")),

            // Literal variants
            "\r\n" | _ if string.len() == 1 => Ok(Terminator::from_str_unchecked(string)),

            // Invalid values
            _ => Err(ConfigError::expected_line_terminator(string) ),
        }
    }    
}

#[derive(Debug)]
pub enum ConfigError {
    ExpectedASCII { string: String },
    ExpectedCharacter { string: String },
    ExpectedLineTerminator { string: String }
}

impl ConfigError {
    pub fn expected_ascii(string: impl Into<String>) -> Self {
        ConfigError::ExpectedASCII { string: string.into() }
    }
    pub fn expected_character(string: impl Into<String>) -> Self {
        ConfigError::ExpectedCharacter { string: string.into() }
    }
    pub fn expected_line_terminator(string: impl Into<String>) -> Self {
        ConfigError::ExpectedLineTerminator { string: string.into() }
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::ExpectedASCII { string } => write!(f, "Expected an ASCII string but found \"{string}\"."),
            ConfigError::ExpectedCharacter { string } => write!(f, "Expected a single character but found string \"{string}\" of length {}.", string.chars().count()),
            ConfigError::ExpectedLineTerminator { string } => write!(f, "Expected a line terminator: \"CRLF\", \"CR\", \"LF\", \"\\r\\n\" or any single character, but found string \"{string}\""),
        }
    }
}

impl std::error::Error for ConfigError {}