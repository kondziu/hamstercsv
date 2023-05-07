pub mod cursed;

pub struct CsvDisplay {
    window: pancurses::Window
}

impl CsvDisplay {
    pub fn new() -> Self {
        let window = pancurses::initscr();
        

        todo!()
    }
}