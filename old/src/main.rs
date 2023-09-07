use minifb::{Key, Scale, Window, WindowOptions};

use core::panic;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
type KeyVec = Rc<RefCell<Vec<u32>>>;

mod cursor;
mod screen_buffer;
struct Input {
    keys: KeyVec,
}

impl Input {callbackec) -> Input {
        Input { keys: data.clone() }
    }
}

impl minifb::InputCallback for Input {
    fn add_char(&mut self, uni_char: u32) {
        self.keys.borrow_mut().push(uni_char);
    }
}

const WIDTH: usize = 740;
const HEIGHT: usize = 480;

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X1,
            resize: false,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_input_callback(Box::new(|x| {

    }));
    window.get_keys().set_input_callback(Box::new(Input::new(&window.get_keys())));

    let mut cursor = cursor::Cursor::new(
        cursor::Font::new("./fira_code.ttf".to_string(), 28.0, screen_buffer::Color::rand()),
        cursor::Boundaries { start_x: 10, start_y: 10, width: 150, height: 75 }
    );

    cursor.print("?asdasdasdasdsadasdad");
    //640x480 (80x8)
    println!("Start render");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut screen_buffer = screen_buffer::ScreenBuffer::new(WIDTH, HEIGHT);
        
        cursor.render(&mut screen_buffer);

        window
            .update_with_buffer(&screen_buffer.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
