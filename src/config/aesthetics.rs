use std::borrow::Borrow;
use std::convert::{TryFrom, Infallible};
use std::ffi::{OsString, OsStr};
use std::fmt::Display;
use std::str::FromStr;

use csscolorparser::{Color, ParseColorError};

use crate::utils::errors::*;

#[derive(clap::Args, Clone, Debug)]
/// Display-specific configuration options
pub struct Aesthetics {    
    #[arg(long, default_value("#002B36"), num_args= 1.., value_delimiter = ':')]
    /// Foreground/text colors for header columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    header_fg_colors: Vec<Rgb>,

    #[arg(long, default_value("#8EA1A1"), num_args= 1.., value_delimiter = ':')]
    /// Background colors for header columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    header_bg_colors: Vec<Rgb>,

    #[arg(long, default_value("#8EA1A1"), num_args= 1.., value_delimiter = ':')]
    /// Foreground/text for value columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    value_fg_colors: Vec<Rgb>,

    #[arg(long, default_values = vec!["#002B36", "#72A0C1"], num_args= 1.., value_delimiter = ':')]
    /// Background for value columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    value_bg_colors: Vec<Rgb>,

    #[arg(long, default_value("#646464"))]
    /// Color used for unfilled cells: single hex or rgb color value
    bg_color: Rgb,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Rgb (csscolorparser::Color);

impl FromStr for Rgb {
    type Err = RgbError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let maybe_color = csscolorparser::parse(value);
        maybe_color
            .map(Rgb::from)
            .map_err(|error| RgbError::ParseColorError {
                string: value.to_owned(),
        reason:            error.to_string()
    })
        
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Color> for Rgb {
    fn from(value: Color) -> Self {
        Rgb(value)
    }
}

// impl TryFrom<&OsStr> for Rgb {
//     type Error = RgbError;

//     fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
//         let error = || Err(RgbError::InvalidOsStr(value.to_owned()));
//         value.to_str().map_or_else(error, Rgb::try_from)
//     }
// }

// impl TryFrom<OsString> for Rgb {
//     type Error = RgbError;

//     fn try_from(value: OsString) -> Result<Self, Self::Error> {
//         let error = || Err(RgbError::InvalidOsStr(value.to_owned()));
//         value.to_str().map_or_else(error, Rgb::try_from)
//     }
// }


impl From<&str> for Rgb {
    fn from(value: &str) -> Self {
        Rgb::from_str(value).unwrap()
    }
}

impl Rgb {
    pub fn parser(value: &str) -> Result<Vec<Rgb>, RgbError> {
        value.split(':').into_iter().map(Rgb::from_str).collect()
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct RgbErrors(Vec<RgbError>);

// impl std::error::Error for RgbErrors {}

// impl Display for RgbErrors {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "Error parsing color definitions:")?;
//         for error in self.0.iter() {
//             writeln!(f, "  - {}", error)?;
//         }
//         Ok(())
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RgbError {
    // InvalidOsStr(OsString),
    ParseColorError{ string: String, reason: String },
}

impl std::error::Error for RgbError {}

impl Display for RgbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // RgbError::InvalidOsStr(os_string) => write!(f, "Cannot parse OS string: {:?}", os_string),
            RgbError::ParseColorError{ string, reason} => write!(f, "Cannot parse color definition {}: {:?}", string, reason),
        }
    }
}