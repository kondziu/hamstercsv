use std::{fmt::Display, collections::HashMap};

use pancurses::{set_title, init_color, init_pair};

use crate::{config::Rgb, utils::errors::OrFailWith};



pub struct Window {
    window: pancurses::Window,
    color_sequence: Sequence,
    colors: HashMap<&'static str, ColorPair>,
}

impl Window {
    pub fn new(title: impl AsRef<str>) -> Self {
        let window = pancurses::initscr();
        set_title(title.as_ref());
        Window { 
            window, 
            color_sequence: Sequence::new(),
            colors: HashMap::new(),
        }
    }

    pub fn color(self) -> Self {
        pancurses::start_color();
        self
    }

    pub fn echo(self, on: bool) -> Self {
        if on {
            pancurses::echo();
        } else {
            pancurses::noecho();
        }
        self
    }

    pub fn blink(self, on: bool) -> Self {
        pancurses::set_blink(on);        
        self
    }

    pub fn keypad(self, on: bool) -> Self {
        self.window.keypad(on);
        self
    }

    pub fn buffer_input(self, on: bool) -> Self {
        if on {
            pancurses::nocbreak();
        } else {
            pancurses::cbreak();
        }
        self
    }

    pub fn add_colors(mut self, key: &'static str, fg: Rgb, bg: Rgb) -> Result<Self, CursedError> {
        let fg_key = self.color_sequence.next()?;        
        self.add_color(fg_key, fg.components());

        let bg_key = self.color_sequence.next()?;
        self.add_color(bg_key, bg.components());

        let pair_key = self.color_sequence.next()?;
        self.add_pair(pair_key, fg_key, bg_key);

        let pair = ColorPair::new(pair_key, fg_key, bg_key);
        self.colors.insert(key, pair);

        Ok(self)
    }

    fn add_color(&self, key: u8, (r, g, b): (u8, u8, u8)) {
        pancurses::init_color(
            key as i16, 
            (r as i16) * 4, 
            (g as i16) * 4, 
            (b as i16) * 4
        );
    }

    fn add_pair(&self, key: u8, fg_key: u8, bg_key: u8) {
        pancurses::init_pair(
            key as i16, 
            fg_key as i16, 
            bg_key as i16
        );
    }
}

struct ColorPair {
    fg: u8,
    bg: u8,
    pair: u8,
}

impl ColorPair {
    pub fn new(pair: u8, fg: u8, bg: u8) -> Self {
        ColorPair { fg, bg, pair }
    }
}

struct Sequence {
    cursor: u8,
}

impl Sequence {
    pub fn new() -> Self {
        Sequence { cursor: 0 }
    }
    pub fn next(&mut self) -> Result<u8, CursedError> {
        { self.cursor == u8::MAX }.or_fail_with(|| CursedError::SequenceExhausted(self.cursor) )?;
        let value = self.cursor;
        self.cursor += 1;
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursedError {
    SequenceExhausted(u8),
}

impl std::error::Error for CursedError {}

impl std::fmt::Display for CursedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedError::SequenceExhausted(value) => write!(f, "Sequence reached value {} and cannot generate another number", value),
        }
    }
}

