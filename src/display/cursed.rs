use std::{fmt::Display, collections::HashMap};

use pancurses::{set_title, init_color, init_pair};

use crate::{config::{Rgb, RgbPair}, utils::errors::{OrFailWith, ToResult}};

#[derive(Debug)]
pub struct Window {
    window: pancurses::Window,
    color_sequence: Sequence,
    colors: HashMap<String, ColorPair>,
}

impl Window {
    pub fn new() -> Self {
        let window = pancurses::initscr();
        Window { 
            window, 
            color_sequence: Sequence::new(),
            colors: HashMap::new(),
        }
    }

    pub fn title(self, title: impl AsRef<str>) -> Self {
        set_title(title.as_ref());
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

    pub fn add_color_pairs(mut self, vector: &Vec<RgbPair>) -> Result<Self, CursedError> {
        for pair in vector.iter() {
            self = self.add_color_pair(pair)?;
        }
        Ok(self)
    }

    pub fn add_color_pair(mut self, pair: &RgbPair) -> Result<Self, CursedError> {
        self.add_colors(pair.name.clone(), pair.fg, pair.bg)
    }

    pub fn add_colors(mut self, key: String, fg: &Rgb, bg: &Rgb) -> Result<Self, CursedError> {
        { !self.colors.contains_key(&key) }
            .or_fail_with(|| CursedError::ColorAlreadyDefined(key.clone()))?;

        if self.colors.is_empty() {
            pancurses::start_color();
        }

        let fg_key = self.color_sequence.next()?;        
        self.add_color(fg_key, fg.components());

        let bg_key = self.color_sequence.next()?;
        self.add_color(bg_key, bg.components());

        let pair_key = self.color_sequence.next()?;
        self.add_pair(pair_key, fg_key, bg_key);

        let pair = ColorPair::new(pair_key, fg_key, bg_key);
        let result = self.colors.insert(key, pair);

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

    pub fn background(self, background_character: char, color_key: &str) -> Result<Self, CursedError> {
        let color = self.colors.get(color_key).into_result(|| CursedError::ColorNotFound(color_key.to_owned()))?;
        let background_attribute = background_character as pancurses::chtype
            | pancurses::COLOR_PAIR(color.pair as u32) as pancurses::chtype;
        self.window.bkgd(background_attribute);
        Ok(self)
    }

    pub fn cursor(self, on: bool) -> Self {
        let visibility = if on { 1 } else { 0 };
        pancurses::curs_set(visibility);
        self
    }

    pub fn clear(self) -> Self {
        self.window.clear();
        self
    }

    pub fn height(&self) -> usize {
        self.window.get_max_y() as usize
    }

    pub fn width(&self) -> usize {
        self.window.get_max_x() as usize
    }

    pub fn height_and_width(&self) -> (usize, usize) {
        let (y, x) = self.window.get_max_yx();
        (y as usize, x as usize)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        pancurses::endwin();
    }
}

#[derive(Debug)]
struct ColorPair {
    _fg: u8,
    _bg: u8,
    pair: u8,
}

impl ColorPair {
    pub fn new(pair: u8, fg: u8, bg: u8) -> Self {
        ColorPair { _fg: fg, _bg: bg, pair }
    }
}

#[derive(Clone, Debug)]
struct Sequence {
    cursor: u8,
}

impl Sequence {
    pub fn new() -> Self {
        Sequence { cursor: 0 }
    }
    pub fn next(&mut self) -> Result<u8, CursedError> {
        { self.cursor < u8::MAX }.or_fail_with(|| CursedError::SequenceExhausted(self.cursor) )?;
        let value = self.cursor;
        self.cursor += 1;
        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CursedError {
    SequenceExhausted(u8),
    ColorNotFound(String),
    ColorAlreadyDefined(String),
}

impl std::error::Error for CursedError {}

impl std::fmt::Display for CursedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedError::SequenceExhausted(value) => write!(f, "Sequence reached value {} and cannot generate another number", value),
            CursedError::ColorNotFound(color_key) => write!(f, "Color {} was not declared", color_key),
            CursedError::ColorAlreadyDefined(color_key) => write!(f, "Cannot declare color {}: already declared", color_key),
        }
    }
}

