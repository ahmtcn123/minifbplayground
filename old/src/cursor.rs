use std::{fs, hash::BuildHasher};
use crate::screen_buffer::{Color, ScreenBuffer};

#[derive(Clone, Debug)]
pub struct Font {
    pub font_size: f32,
    pub font: Vec<u8>,
    pub color: Color,
    pub builded: fontdue::Font
}

impl Font {
    pub fn new(font_dir: String, font_size: f32, color: Color) -> Font {
        let file = fs::read(font_dir);
        match file {
            Ok(file) => Font {
                font_size,
                font: file.clone(),
                color,
                builded: fontdue::Font::from_bytes(file, fontdue::FontSettings::default()).unwrap()
            },
            Err(e) => panic!("Failed to read font file ({})", e.to_string()),
        }
    }
}

pub struct Boundaries {
    pub start_x: usize,
    pub start_y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct Cursor {
    pub font: Font,
    pub case: bool,
    pub buffer: Vec<char>,
    pub pos: usize,
    pub blink: bool,
    pub boundaries: Boundaries
}

impl Cursor {
    pub fn new(font: Font, boundaries: Boundaries) -> Cursor {
        Cursor {
            font,
            case: true,
            buffer: vec![],
            pos: 0,
            blink: true,
            boundaries,
        }
    }

    pub fn new_line(&mut self) {
        self.pos = 0;
        self.buffer.push('\n');
    }

    pub fn println(&mut self, text: &str) {
        self.print(text);
        self.new_line();
    }

    pub fn print(&mut self, text: &str) {
        for c in text.chars() {
            self.print_char(c);
        }
    }

    fn print_char(&mut self, c: char) {
        self.buffer.push(c);
        self.pos += 1;
    }



    pub fn backspace(&mut self) {
        if self.pos > 0 {
            self.buffer.remove(self.pos - 1);
            self.pos -= 1;
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.pos = 0;
    }


    pub fn render(&mut self, screen_buffer: &mut ScreenBuffer) {
        let mut rx = 0;
        let mut ry = 0;
        for char in &self.buffer {
            //screen_buffer.draw_rect(self.boundaries.start_x + rx, self.boundaries.start_y + ry, 8, 80, Color::rand());
            let (h, w) = screen_buffer.draw_char(*char, self.boundaries.start_x + rx, self.boundaries.start_y + ry, self.font.clone());
            match char {
                '\n' => {
                    rx = 0;
                    ry += 80;
                },
                _ => {
                    rx += 15;
                }
            }
            
        }
    }
}