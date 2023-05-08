use crate::config::Config;

pub mod cursed;

pub struct CsvDisplay {
    window: cursed::Window,
}

impl CsvDisplay {
    pub fn new(config: &Config) -> Result<Self, DisplayError> {


        let mut window = cursed::Window::new().title(config.path_str())
            .blink(false)
            .echo(false)
            .keypad(true)
            .buffer_input(false)
            .cursor(false)
            .add_color_pair(&config.aesthetics.basic_color()).map_err(DisplayError::CannotCreateWindow)?
            .add_color_pairs(&config.aesthetics.header_colors()).map_err(DisplayError::CannotCreateWindow)?
            .add_color_pairs(&config.aesthetics.value_colors()).map_err(DisplayError::CannotCreateWindow)?
            ;

        let display = CsvDisplay { window };
        Ok(display)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayError {
    CannotCreateWindow(cursed::CursedError)
}

impl std::error::Error for DisplayError {}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::CannotCreateWindow(reason) => write!(f, "Cannot create window: {}", reason),
        }
    }
}



impl From<cursed::CursedError> for DisplayError {
    fn from(e: cursed::CursedError) -> Self {
        DisplayError::CannotCreateWindow(e)
    }
}
