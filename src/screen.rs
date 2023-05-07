use crate::csv::*;

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

pub struct CSVDisplay {
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

    cell_dimensions: CellDimentions,
    
    csv: CSVFile,    
}
impl CSVDisplay {
    pub fn from(csv: CSVFile, options: &()) -> Self {

        ncurses::setlocale(ncurses::LcCategory::all, ""/*options.locale.as_str()*/); // TODO is this actually configurable to any reasonable extent?

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

        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        ncurses::clear();

        let mut display = CSVDisplay { 
            csv, 

            first_row: 0, 
            first_column: 0, 

            last_row: 0, 
            last_column: 0, 

            visible_columns: 0,
            visible_rows: 0,

            column_width: 10, 
            row_height: 2, 

            screen_height: 0,
            screen_width: 0,

            cell_dimensions: CellDimentions { height: 0, width: 0 }
        };

        display.measure_screen();
        display
    }

    fn measure_screen(&mut self) -> bool {
        let mut screen_height: i32 = 0;
        let mut screen_width: i32 = 0;

        ncurses::getmaxyx(ncurses::stdscr(), &mut screen_height, &mut screen_width);

        if self.screen_height == screen_height as usize && self.screen_width  == screen_width as usize {
            return false;
        } else {
            self.screen_height = screen_height as usize;
            self.screen_width = screen_width as usize;
            return true;
        }
    }

    fn figure_out_which_rows_to_display(&mut self) -> () {
        self.visible_rows = self.screen_height / self.row_height - 1 /* headers */ - 1 /* status bar */; 
        self.last_row = std::cmp::min(self.first_row + self.visible_rows, self.csv.row_count());

        log::info!("Displaying rows: {}..{} (total: {} rows)", 
                   self.first_row, self.last_row, self.csv.row_count());
    }

    fn figure_out_which_columns_to_display(&mut self) -> () {
        self.visible_columns = self.screen_width / self.column_width;
        self.last_column = self.first_column + self.visible_columns;

        log::info!("Displaying columns: {}..{} (total: {} columns", 
                   self.first_column, self.last_column, self.csv.column_count());
    }

    fn figure_out_cell_dimensions(&mut self) -> () {
        self.cell_dimensions = CellDimentions { width: self.column_width - 1, height: self.row_height };
    }

    fn display_column_header(&self, column_index: usize, column: &CSVColumn) {       

        let text = column.header().to_owned()
            .cut_or_pad_to(self.cell_dimensions.width, " ")
            .join("");

        let x = ((column_index - self.first_column) * self.column_width) as i32;
        let y = 0;

        let is_even = column_index % 2 == 0;
        let colors = if is_even { COLOR_HEADER_PAIR_EVEN } else { COLOR_HEADER_PAIR_ODD };

        log::info!("display header, column_index: {}, color: {}", column_index, colors);

        ncurses::attron(ncurses::A_BOLD());                
        ncurses::attron(ncurses::COLOR_PAIR(colors));        
        ncurses::mv(y, x);        
        ncurses::addstr(text.as_str());
        ncurses::addstr(" ");                            
        ncurses::attroff(ncurses::A_BOLD());                
        ncurses::attroff(ncurses::COLOR_PAIR(colors));
    }

    fn display_column_values(&self, column_index: usize, column: &CSVColumn) -> () {

        let is_even = column_index % 2 == 0;
        let colors = if is_even { COLOR_VALUES_PAIR_EVEN } else { COLOR_VALUES_PAIR_ODD };

        let cells = (self.first_row..self.last_row)
            .map(|row_index| {
                column.value(row_index)
                    .map_or_else(Vec::new, |csv_item| {
                        csv_item.cut_or_pad_to(self.cell_dimensions, PADDING)
                            .into_iter()
                            .map(|vector| {
                                vector.join("")
                            }).collect()
                    })                    
            });

        ncurses::attron(ncurses::COLOR_PAIR(colors));
        
        let mut y = 1;                
        for row_lines in cells {
            let x = ((column_index - self.first_column) * self.column_width) as i32;

            for row_line in row_lines {  
                ncurses::mv(y, x);
                ncurses::addstr(row_line.as_str());
                ncurses::addstr(PADDING);

                y += 1;
            }
        } 

        ncurses::attroff(ncurses::COLOR_PAIR(colors));
    }

    pub fn run(&mut self) {

        loop {

            self.measure_screen();
            self.figure_out_which_rows_to_display();
            self.figure_out_which_columns_to_display();
            self.figure_out_cell_dimensions();
                        

            for column_index in self.first_column..self.last_column {

                log::info!("column_index: {}", column_index);

                if let Some(column) = self.csv.get_column(column_index) {
                    self.display_column_header(column_index, column);
                    self.display_column_values(column_index, column);
                }
            }

            ncurses::mv(self.screen_height as i32 - 1, 0);
            ncurses::addstr(&format!("row: {}-{}, cols: {}-{}", self.first_row, self.last_row, self.first_column, self.last_column));

            let input = ncurses::get_wch().unwrap();

            match input {
                ncurses::WchResult::KeyCode(value) => {
                    let bytes = value.to_be_bytes();                                        
                    log::info!("key input: {:?}", bytes);
                    match bytes {
                        [0, 0, 1, 2] => if self.last_row < self.csv.row_count() { self.first_row += 1 }, // DOWN
                        [0, 0, 1, 3] => if self.first_row > 0 { self.first_row -= 1 }, // UP
                        [0, 0, 1, 4] => if self.first_column > 0 { self.first_column -= 1 }, // LEFT
                        [0, 0, 1, 5] => if self.last_column < self.csv.column_count() { self.first_column += 1 }, // RIGHT
                        _ => (),                        
                    }
                }
                ncurses::WchResult::Char(value) => {
                    let bytes = value.to_be_bytes();
                    let characters = [bytes[0] as char, bytes[1] as char, bytes[2] as char, bytes[3] as char];
                    log::info!("char input: {:?}", characters);
                    match characters {
                        [ '\0', '\0', '\0', 'q' ] => break,
                        _ => (),
                    }
                }
            }        
        }
    }
}

impl Drop for CSVDisplay {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}