use crate::{config, data, utils::errors::OrFailWith};

pub mod cursed;

#[derive(Debug)]
pub struct CsvDisplay<'a> {
    window: cursed::Window,
    viewport: Viewport,
    data: &'a data::Csv,
}

impl<'a> CsvDisplay<'a> {
    pub fn new(config: &config::Config, data: &'a data::Csv) -> Result<Self, DisplayError> {

        let basic_color = config.aesthetics.basic_color();
        let header_colors = config.aesthetics.header_colors();
        let value_colors = config.aesthetics.value_colors();

        let window = cursed::Window::new().title(config.path_str())
            .blink(false)
            .echo(false)
            .keypad(true)
            .buffer_input(false)
            .cursor(false)
            .add_color_pair(&basic_color).map_err(DisplayError::CannotCreateWindow)?
            .add_color_pairs(&header_colors).map_err(DisplayError::CannotCreateWindow)?
            .add_color_pairs(&value_colors).map_err(DisplayError::CannotCreateWindow)?;

        let viewport = Viewport::new(config, &data, &window);

        Ok(CsvDisplay { window, viewport, data })
    }
}

#[derive(Clone, Debug)]
pub struct Viewport {
    first_column: usize,
    last_column: usize, // Invariant last_column >= first_column
    first_row: usize,
    last_row: usize, // Invariant last_row >= first_row
    visible_columns: usize,
    visible_rows: usize,
    column_width: usize, // Invariant: > 0 & <= screen_width
    row_height: usize,   // Invariant: > 0 & <= screen_height - 2
    screen_height: usize, 
    screen_width: usize,
    row_count: usize,
    column_count: usize,
}

impl Viewport {
    pub fn new(config: &config::Config, data: &data::Csv, window: &cursed::Window) -> Self {
        
        let (screen_height, screen_width) = window.height_and_width();

        let column_width = config.aesthetics.column_width as usize;
        let column_count = data.row_count();
        let first_visible_column = 0;
        let visible_columns = Self::calculate_visible_columns(screen_width, column_width);
        let last_visible_column = Self::calculate_last_visible_column_from_first(first_visible_column, visible_columns, column_count);
        
        let row_height = config.aesthetics.row_height as usize;
        let row_count = data.row_count();
        let first_visible_row = 0;
        let visible_rows = Self::calculate_visible_rows(screen_height, row_height);
        let last_visible_row = Self::calculate_last_visible_row_from_first(first_visible_row, visible_rows, row_count);

        Viewport { 
            screen_height,
            screen_width, 

            column_width, 
            column_count,
            first_column: first_visible_column, // rename to first_visible_column
            last_column: last_visible_column,   // rename to last_visible_column
            visible_columns, 

            row_height, 
            row_count, 
            first_row: first_visible_row,  // rename to first_visible_row 
            last_row: last_visible_row,    // rename to last_visible_row
            visible_rows, 
        }
    }

    fn calculate_visible_rows(screen_height: usize, row_height: usize) -> usize {
        screen_height / row_height - 1 /* headers */ - 1 /* status bar */
    }

    fn calculate_last_visible_row_from_first(first_visible_row: usize, visible_rows: usize, row_count: usize) -> usize {
        std::cmp::min(first_visible_row + visible_rows, row_count)
    }

    fn calculate_first_visible_row_from_last(last_visible_row: usize, visible_rows: usize) -> usize {
        if last_visible_row > visible_rows {
            last_visible_row - visible_rows
        } else {
            0
        }
    }

    fn calculate_visible_columns(screen_width: usize, column_width: usize) -> usize {
        screen_width / column_width
    }

    fn calculate_last_visible_column_from_first(first_visible_column: usize, visible_columns: usize, column_count: usize) -> usize {
        std::cmp::min(first_visible_column + visible_columns, column_count)
    }

    fn calculate_first_visible_column_from_last(last_visible_column: usize, visible_columns: usize) -> usize {
        if last_visible_column > visible_columns {
            last_visible_column - visible_columns
        } else {
            0
        }
    }

    // Move viewport to specific row. If the target row is out of bounds of the
    // data, go to the last row.
    pub fn jump_to_row(&mut self, target_row: usize) {
        // TODO: Only needs recalculating if screen height or row height changed
        self.visible_rows = Self::calculate_visible_rows(self.screen_height, self.row_height);
        self.last_row = Self::calculate_last_visible_row_from_first(target_row, self.visible_rows, self.row_count);
        self.first_row = Self::calculate_first_visible_row_from_last(self.last_row, self.visible_rows);
    }

    // Move viewport to specific column
    pub fn jump_to_column(&mut self, target_column: usize) {
        // TODO: Only needs recalculating if screen width or column width changed
        self.visible_columns = Self::calculate_visible_columns(self.screen_width, self.column_width);
        self.last_column = Self::calculate_last_visible_column_from_first(target_column, self.visible_columns, self.column_count);
        self.first_column = Self::calculate_first_visible_column_from_last(self.last_column, self.visible_columns);
    }

    // Move viewport from the current row to row + row_offset
    pub fn shift_by_rows(&mut self, row_offset: isize) {
        let target_row = if row_offset > 0 {
            self.first_row + row_offset as usize
        } else {
            self.first_row - (-row_offset) as usize
        };
        self.jump_to_row(target_row)
    }

    // Move viewport from current column to column + column_offset
    pub fn shift_by_columns(&mut self, column_offset: isize) {
        let target_column = if column_offset > 0 {
            self.first_column + column_offset as usize
        } else {
            self.first_column - (-column_offset) as usize
        };
        self.jump_to_column(target_column)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayError {
    CannotCreateWindow(cursed::CursedError),
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
