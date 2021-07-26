use crate::{cli::Options, csv::*};

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

pub struct CSVDisplay {
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