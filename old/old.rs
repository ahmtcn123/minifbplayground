        /*
        let mut screen_buffer = ScreenBuffer::new(WIDTH, HEIGHT);
        let (h, w) = screen_buffer.draw_char(
            't',
            0,
            1,
            Font::new("./Montserrat-Light.ttf".to_string(), 25.0, Color::rand()),
        );

        screen_buffer.draw_char(
            'e',
            1 + w,
            1,
            Font::new("./Montserrat-Light.ttf".to_string(), 25.0, Color::rand()),
        );

        let (mouse_x, mouse_y) = window.get_mouse_pos(minifb::MouseMode::Pass).unwrap();
        let modified_buffer = cursor.render(screen_buffer.clone());
        screen_buffer.buffer = modified_buffer.buffer;
        */

        let mut decoder = png::Decoder::new(File::open("./Skeleton.png").unwrap());
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info().unwrap();
        let mut img_data = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut img_data).unwrap();

        //let new_img = screen_buffer.resize_image(img_data, info.width as usize / 2, info.height as usize / 2);


        screen_buffer.draw_image(
            img_data,
            info.width as usize,
            info.height as usize,
            if info.color_type == Rgb {
                0
            } else if info.color_type == Rgba {
                1
            } else {
                unimplemented!("Color type not implemented; {:?}", info.color_type)
            },
        );

        //io::copy(&mut reader, &mut screen_buffer).unwrap();

        //Info: OutputInfo { width: 58, height: 58, color_type: Rgba, bit_depth: Eight, line_size: 232 }

        /*
        let mut draw_str = |x: &str| {
            let mut last_width = 0;
            for char in x.chars() {
                if char == ' ' {
                    last_width += 6;
                    continue;
                }
                last_width += screen_buffer
                    .draw_char(
                        char,
                        last_width + 6,
                        1,
                        Font::new("./Montserrat-Light.ttf".to_string(), 25.0, Color::rand()),
                    )
                    .1;
            }
        };


        let dt_nano = Utc::now();
        draw_str(&format!("{}", dt_nano.format("%H:%M:%S")));
        */


        let mut cursor = Cursor::new(
            Font::new("./Montserrat-Light.ttf".to_string(), 25.0, Color::rand()),
            Boundaries {
                start_x: 15,
                start_y: 15,
                width: WIDTH,
                height: HEIGHT,
            },
        );