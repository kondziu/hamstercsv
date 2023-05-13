use std::cmp::max;
use std::fmt::Display;
use std::str::FromStr;

use csscolorparser::Color;

#[derive(clap::Args, Clone, Debug)]
/// Display-specific configuration options
pub struct Aesthetics {    
    #[arg(long, default_value("#002B36"), num_args= 1.., value_delimiter = ':')]
    /// Foreground/text colors for header columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    pub(crate) header_fg_colors: Vec<Rgb>,

    #[arg(long, default_value("#8EA1A1"), num_args= 1.., value_delimiter = ':')]
    /// Background colors for header columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    pub(crate) header_bg_colors: Vec<Rgb>,

    #[arg(long, default_value("#8EA1A1"), num_args= 1.., value_delimiter = ':')]
    /// Foreground/text for value columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    pub(crate) value_fg_colors: Vec<Rgb>,

    #[arg(long, default_values = vec!["#002B36", "#72A0C1"], num_args= 1.., value_delimiter = ':')]
    /// Background for value columns: colon-separated list of hex or rgb
    /// color values, colors repeat periodially for consecutive columns)
    pub(crate) value_bg_colors: Vec<Rgb>,

    #[arg(long, default_value("#646464"))]
    /// Color used for unfilled cells: single hex or rgb color value
    pub(crate) bg_color: Rgb,

    #[arg(long, default_value_t = 10)]
    /// Width of displayed CSV columns (in characters)
    pub(crate) column_width: u8,

    #[arg(long, default_value_t = 2)]
    /// Height of displayed CSV rows (in lines)
    pub(crate) row_height: u8,
}

impl Aesthetics {
    pub fn basic_color<'a>(&'a self) -> RgbPair<'a> {
        RgbPair { name: 
            "basic".to_owned(),
            fg: &self.bg_color, 
            bg: &self.bg_color 
        }
    }

    pub fn header_colors<'a>(&'a self) -> Vec<RgbPair<'a>>{
        RgbPair::bind(
            |i| format!("header_{}", i), 
            &self.header_fg_colors, 
            &self.header_bg_colors
        )
    }

    pub fn value_colors<'a>(&'a self) -> Vec<RgbPair<'a>>{
        RgbPair::bind(
            |i| format!("value_{}", i), 
            &self.value_fg_colors, 
            &self.value_bg_colors
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RgbPair<'a> { pub name: String, pub fg: &'a Rgb, pub bg: &'a Rgb }

impl<'a> RgbPair<'a> {
    pub fn new(name: impl Into<String>, fg: &'a Rgb, bg: &'a Rgb) -> Self {
        RgbPair { name: name.into(), fg, bg }
    }

    pub fn bind<'b, F>(name_generator: F, fg_vector: &'b Vec<Rgb>, bg_vector: &'b Vec<Rgb>)  -> Vec<RgbPair<'b>> where F: Fn(usize) -> String {
        let mut pairs = Vec::new();
        let bg_length = bg_vector.len();
        let fg_length = fg_vector.len();
        let range = 0..max(fg_length, bg_length);

        for i in range {
            let bg_color = bg_vector.get(i % bg_length).unwrap();
            let fg_color = fg_vector.get(i % fg_length).unwrap();
            let pair = RgbPair::new(name_generator(i), fg_color, bg_color);
            pairs.push(pair)
        }

        pairs
    }
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

    pub fn components(&self) -> (u8,u8,u8) {
        let (r,g,b,_) = self.0.to_linear_rgba_u8();
        (r,g,b)
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