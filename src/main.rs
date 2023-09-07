mod cursor;
mod screen;

use cursor::{Boundaries, Cursor, Font};
use minifb::{Key, MouseButton, Scale, ScaleMode, Window, WindowOptions};
use screen::{Color, ScreenBuffer};
use std::f32::consts::E;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::sync::{mpsc::channel, Arc};
use std::{fs, thread};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    // Limit to max ~60 fps update rate
    //window.limit_update_rate(Some(std::time::Duration::from_micros(1)));

    let file = fs::read("./fira_code.ttf".to_string()).unwrap();
    let font = Font::from_bytes(file, 18.0);

    let mut cursor = Cursor::new(
        font,
        Boundaries {
            start_x: 15,
            start_y: 15,
            width: WIDTH,
            height: HEIGHT,
        },
    );

    let screen = Arc::new(Mutex::new(ScreenBuffer::new(WIDTH, HEIGHT)));
    let screen_updated = Arc::new(AtomicBool::new(true));

    enum MessageType {
        Render(usize),
        MouseMove(usize, usize),
        MouseDown(MouseButton),
        MouseUp(MouseButton),
        MouseScroll((f32, f32)),
    }

    let (tx, rx) = channel::<MessageType>();

    let render_thread = std::thread::spawn({
        let screen = Arc::clone(&screen);
        let screen_updated = Arc::clone(&screen_updated);

        move || {
            println!("Starting render thread");
            let mut window = Window::new(
                "Test - ESC to exit",
                WIDTH,
                HEIGHT,
                WindowOptions {
                    resize: false,
                    scale: Scale::X1,
                    scale_mode: ScaleMode::UpperLeft,
                    ..Default::default()
                },
            )
            .unwrap();

            let mut time_start = std::time::Instant::now();
            let mut frame_rendered = 0;
            let tx_clone = tx.clone();
            let mut fps = 0;

            while window.is_open() && !window.is_key_down(Key::Escape) {
                if screen_updated.load(Ordering::Relaxed) {
                    screen_updated.store(false, Ordering::Relaxed);

                    if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                        tx_clone
                            .send(MessageType::MouseMove(x as usize, y as usize))
                            .unwrap();
                    }

                    if window.get_mouse_down(minifb::MouseButton::Left) {
                        tx_clone
                            .send(MessageType::MouseDown(MouseButton::Left))
                            .unwrap();
                    } else {
                        tx_clone
                            .send(MessageType::MouseUp(MouseButton::Left))
                            .unwrap();
                    }

                    if window.get_mouse_down(minifb::MouseButton::Right) {
                        tx_clone
                            .send(MessageType::MouseDown(MouseButton::Right))
                            .unwrap();
                    } else {
                        tx_clone
                            .send(MessageType::MouseUp(MouseButton::Right))
                            .unwrap();
                    }

                    if let Some(scroll) = window.get_scroll_wheel() {
                        tx_clone.send(MessageType::MouseScroll(scroll)).unwrap();
                    }

                    let screen_lock = screen.lock().unwrap();
                    window
                        .update_with_buffer(&screen_lock.buffer, WIDTH, HEIGHT)
                        .unwrap();

                    let sec = time_start.elapsed().as_secs();
                    if sec > 0 {
                        fps = frame_rendered / sec as usize;
                        frame_rendered = 0;
                        time_start = std::time::Instant::now();
                    } else {
                        frame_rendered += 1;
                    }

                    tx_clone.send(MessageType::Render(fps)).unwrap();
                }

                thread::yield_now();
            }
        }
    });

    struct Element {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    }

    let mut elements: Vec<Element> = vec![
        Element {
            x: 0,
            y: HEIGHT - 50,
            width: 50,
            height: 50,
            color: Color::red(),
        },
        Element {
            x: 50,
            y: HEIGHT - 50,
            width: 50,
            height: 50,
            color: Color::green(),
        },
        Element {
            x: 100,
            y: HEIGHT - 50,
            width: 50,
            height: 50,
            color: Color::blue(),
        },
        Element {
            x: 150,
            y: HEIGHT - 50,
            width: 50,
            height: 50,
            color: Color::from_rgb(255, 255, 255),
        },
        Element {
            x: 225,
            y: HEIGHT - 25,
            width: 25,
            height: 25,
            color: Color::from_rgb(0, 255, 255),
        },
        Element {
            x: 300,
            y: HEIGHT - 25,
            width: 25,
            height: 25,
            color: Color::from_rgb(0, 255, 0),
        },
        Element {
            x: 350,
            y: HEIGHT - 25,
            width: 25,
            height: 25,
            color: Color::from_rgb(255, 0, 0),
        },
    ];
    let mut global_color = Color::from_rgb(255, 255, 255);

    let input_thread = std::thread::spawn(move || {
        let mut mouse_pos = (0, 0);
        let mut mouse_scroll = (0_f32, 0_f32);
        let mut right_click = false;
        let mut left_click = false;
        let mut background_select = false;
        let mut background_color = Color::from_rgb(0, 0, 0);
        let mut button_clicked = false;
        while let Ok(msg) = rx.recv() {
            let mut screen_lock = screen.lock().unwrap();
            match msg {
                MessageType::Render(fps) => {
                    cursor.clear(&mut screen_lock);
                    cursor.println(&format!("FPS: {}", fps));
                    cursor.println(&format!("X: {}, Y: {}", mouse_pos.0, mouse_pos.1));
                    cursor.println(&format!("Left click: {}", left_click));
                    cursor.println(&format!("Right click: {}", right_click));
                    cursor.println(&format!(
                        "Scroll to (x: {:?}, y: {:?})",
                        mouse_scroll.0 as usize, mouse_scroll.1 as usize
                    ));
                    if background_select {
                        cursor.println(&format!("\nSelecting Background Color"));
                    }
                    cursor.render(&mut screen_lock);

                    for element in elements.iter() {
                        screen_lock.draw_rect(
                            element.x,
                            element.y,
                            element.width,
                            element.height,
                            element.color,
                        );
                    }

                    for i in 0..WIDTH {
                        screen_lock.put_pixel(i, 10, Color::rand())
                    }

                    if left_click {
                        let clicked_element = elements.iter().position(|element| {
                            element.x <= mouse_pos.0
                                && element.x + element.width >= mouse_pos.0
                                && element.y <= mouse_pos.1
                                && element.y + element.height >= mouse_pos.1
                        });

                        if let Some(index) = clicked_element {
                            if !button_clicked {
                                button_clicked = true;
                                eprintln!("Element grabbed at {}", index);
                                if index == 4 {
                                    background_select = !background_select;
                                } else {
                                    if background_select {
                                        background_color = elements[index].color;
                                        cursor.background_color = elements[index].color;
                                    } else {
                                        cursor.color = elements[index].color;
                                    }
                                    global_color = elements[index].color;
                                    cursor.clear(&mut screen_lock);
                                    screen_lock.clear(background_color);
                                }
                            }
                        } else {
                            screen_lock.draw_rect(mouse_pos.0, mouse_pos.1, 10, 10, global_color);
                        }
                    } else {
                        button_clicked = false;
                    }

                    if right_click {
                        screen_lock.draw_rect(
                            mouse_pos.0,
                            mouse_pos.1,
                            10,
                            10,
                            Color::from_rgb(0, 0, 0),
                        );
                    }

                    screen_updated.store(true, Ordering::Relaxed);
                }
                MessageType::MouseMove(x, y) => {
                    mouse_pos = (x, y);
                }
                MessageType::MouseDown(button) => {
                    if button == MouseButton::Left {
                        left_click = true;
                    } else if button == MouseButton::Right {
                        right_click = true;
                    }
                }
                MessageType::MouseUp(button) => {
                    if button == MouseButton::Left {
                        left_click = false;
                    } else if button == MouseButton::Right {
                        right_click = false;
                    }
                }
                MessageType::MouseScroll(button) => {
                    mouse_scroll = button;
                }
            }
        }
    });

    render_thread.join().unwrap();
    input_thread.join().unwrap();
}
