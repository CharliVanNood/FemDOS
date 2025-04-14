use crate::{filesystem::{self, create_file, file_exists, get_current_flow, update_file}, infoln, input, string::BigString, vec::BigVec, window::{BUFFER_HEIGHT, BUFFER_WIDTH, SCREEN_WRITER}};

pub fn render_background(name: [u8; 20]) {
    let window_offset_x = 160;

    let header_color = SCREEN_WRITER.lock().get_rgb(100, 100, 100);
    for x in 0..BUFFER_WIDTH - window_offset_x {
        for y in 0..BUFFER_HEIGHT {
            if y > 10 {
                SCREEN_WRITER.lock().set_pixel(x + window_offset_x, y, 15);
            } else {
                SCREEN_WRITER.lock().set_pixel(x + window_offset_x, y, header_color);
            }
        }
    }

    for character in name.iter().enumerate() {
        if *character.1 == 0 { break; }
        SCREEN_WRITER.lock().draw_character(*character.1, 5 + window_offset_x + 6 * character.0, 2, 15, header_color);
    }
}

pub fn update_text(text: &mut BigString) -> [[u8; 25]; 128] {
    let window_offset_x = 160;

    let mut character_position_x = 0;
    let mut character_position_y = 0;

    let mut screen_buffer = [[0; 25]; 128];

    for i in 0..text.len() {
        let character = text.get(i);

        if character as u8 == b'\n' {
            character_position_x = 0;
            character_position_y += 1;
        }

        SCREEN_WRITER.lock().draw_character(
            character as u8, 
            5 + window_offset_x + 6 * character_position_x, 
            12 + 10 * character_position_y, 
            0, 15
        );
        screen_buffer[character_position_y][character_position_x] = character as u8;

        character_position_x += 1;
        if character_position_x == 25 {
            character_position_x = 0;
            character_position_y += 1;
        }
    }

    screen_buffer
}

fn remove_caret(x: usize, y: usize) {
    for i in 0..5 {
        for j in 0..3 {
            let current_pixel_color = SCREEN_WRITER.lock().get_pixel(x + i, y + j + 4);
            SCREEN_WRITER.lock().set_pixel(x + i, y + j + 4, current_pixel_color - 1);
        }
    }
}

fn draw_caret(x: usize, y: usize) {
    for i in 0..5 {
        for j in 0..3 {
            let current_pixel_color = SCREEN_WRITER.lock().get_pixel(x + i, y + j + 4);
            SCREEN_WRITER.lock().set_pixel(x + i, y + j + 4, current_pixel_color + 1);
        }
    }
}

pub fn run(mut screen_buffer: [[u8; 25]; 128], name: [u8; 20]) {
    let window_offset_x = 160;

    let mut previous_cursor_x = 0;
    let mut previous_cursor_y = 0;
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    let mut interval = 50;
    let mut caret_state = false;

    let mut running = true;

    input::KEYPRESSES.lock().0 = [0; 8];
    input::KEYPRESSES.lock().1 = 0;

    while running {
        let keypresses = {
            let lock = input::KEYPRESSES.lock();
            lock.clone()
        };

        for key in keypresses.0 {
            if key == 0 {
                interval -= 1;
                if interval == 0 {
                    interval = 50;
                    caret_state = !caret_state;
                    if caret_state == true {
                        draw_caret(
                            5 + window_offset_x + 6 * cursor_x, 
                            12 + 10 * cursor_y
                        );
                    } else {
                        remove_caret(
                            5 + window_offset_x + 6 * cursor_x, 
                            12 + 10 * cursor_y
                        );
                    }
                }
                continue;
            }

            if key == 27 {
                running = false;
                break;
            }

            if key == 256 {
                previous_cursor_x = cursor_x;
                previous_cursor_y = cursor_y;
                if cursor_y > 0 {
                    cursor_y -= 1;
                }
                draw_caret(
                    5 + window_offset_x + 6 * cursor_x, 
                    12 + 10 * cursor_y
                );
                if caret_state {
                    remove_caret(
                        5 + window_offset_x + 6 * previous_cursor_x, 
                        12 + 10 * previous_cursor_y
                    );
                }
                caret_state = true;
                continue;
            }
            if key == 257 {
                previous_cursor_x = cursor_x;
                previous_cursor_y = cursor_y;
                if cursor_y < 127 {
                    cursor_y += 1;
                }
                draw_caret(
                    5 + window_offset_x + 6 * cursor_x, 
                    12 + 10 * cursor_y
                );
                if caret_state {
                    remove_caret(
                        5 + window_offset_x + 6 * previous_cursor_x, 
                        12 + 10 * previous_cursor_y
                    );
                }
                caret_state = true;
                continue;
            }
            if key == 258 {
                previous_cursor_x = cursor_x;
                previous_cursor_y = cursor_y;
                if cursor_x > 0 {
                    cursor_x -= 1;
                }
                draw_caret(
                    5 + window_offset_x + 6 * cursor_x, 
                    12 + 10 * cursor_y
                );
                if caret_state {
                    remove_caret(
                        5 + window_offset_x + 6 * previous_cursor_x, 
                        12 + 10 * previous_cursor_y
                    );
                }
                caret_state = true;
                continue;
            }
            if key == 259 {
                previous_cursor_x = cursor_x;
                previous_cursor_y = cursor_y;
                cursor_x += 1;
                if cursor_x == 25 {
                    cursor_x = 0;
                    if cursor_y < 127 {
                        cursor_y += 1;
                    }
                }
                draw_caret(
                    5 + window_offset_x + 6 * cursor_x, 
                    12 + 10 * cursor_y
                );
                if caret_state {
                    remove_caret(
                        5 + window_offset_x + 6 * previous_cursor_x, 
                        12 + 10 * previous_cursor_y
                    );
                }
                caret_state = true;
                continue;
            }

            if key as u8 == b'\n' {
                screen_buffer[cursor_y][cursor_x] = key as u8;
                previous_cursor_x = cursor_x;
                previous_cursor_y = cursor_y;
                cursor_x = 0;
                if cursor_y < 127 {
                    cursor_y += 1;
                }
                draw_caret(
                    5 + window_offset_x + 6 * cursor_x, 
                    12 + 10 * cursor_y
                );
                if caret_state {
                    remove_caret(
                        5 + window_offset_x + 6 * previous_cursor_x, 
                        12 + 10 * previous_cursor_y
                    );
                }
                continue;
            }
            if key == 8 {
                if cursor_x > 0 {
                    previous_cursor_x = cursor_x;
                    previous_cursor_y = cursor_y;
                    cursor_x -= 1;

                    SCREEN_WRITER.lock().draw_character(
                        0, 
                        5 + window_offset_x + 6 * cursor_x, 
                        12 + 10 * cursor_y, 
                        0, 15
                    );
                } else if cursor_y > 0 {
                    previous_cursor_x = cursor_x;
                    previous_cursor_y = cursor_y;
                    cursor_x = 24;
                    if cursor_y > 0 {
                        cursor_y -= 1;
                    }

                    SCREEN_WRITER.lock().draw_character(
                        0, 
                        5 + window_offset_x + 6 * cursor_x, 
                        12 + 10 * cursor_y, 
                        0, 15
                    );
                }

                screen_buffer[cursor_y][cursor_x] = 0;
                draw_caret(
                    5 + window_offset_x + 6 * cursor_x, 
                    12 + 10 * cursor_y
                );
                if caret_state {
                    remove_caret(
                        5 + window_offset_x + 6 * previous_cursor_x, 
                        12 + 10 * previous_cursor_y
                    );
                }
                caret_state = true;
                continue;
            }

            SCREEN_WRITER.lock().draw_character(
                key as u8, 
                5 + window_offset_x + 6 * cursor_x, 
                12 + 10 * cursor_y, 
                0, 15
            );
            screen_buffer[cursor_y][cursor_x] = key as u8;
            cursor_x += 1;

            if cursor_x == 25 {
                previous_cursor_x = cursor_x - 1;
                previous_cursor_y = cursor_y;
                cursor_x = 0;
                if cursor_y < 127 {
                    cursor_y += 1;
                }
            }

            draw_caret(
                5 + window_offset_x + 6 * cursor_x, 
                12 + 10 * cursor_y
            );
            caret_state = true;
        }

        input::KEYPRESSES.lock().0 = [0; 8];
        input::KEYPRESSES.lock().1 = 0;

        x86_64::instructions::hlt();
    }

    for x in 0..BUFFER_WIDTH - window_offset_x {
        for y in 0..BUFFER_HEIGHT {
            SCREEN_WRITER.lock().set_pixel(x + window_offset_x, y, 215);
        }
    }

    write_to_file(screen_buffer, name);
}

pub fn write_to_file(data: [[u8; 25]; 128], name: [u8; 20]) {
    let mut data_vec = BigVec::new();

    for line in data {
        let mut character_found = false;
        for character in line {
            if character != 0 { character_found = true; }
        }
        if !character_found { continue; }

        for character in line {
            if character == 0 {
                data_vec.add(b' ' as usize);
                continue;
            }
            if character == b'\n' {
                data_vec.add(character as usize);
                break;
            }
            data_vec.add(character as usize);
        }
    }

    if !file_exists(name) {
        create_file(get_current_flow(), name, "txt", data_vec);
    } else {
        update_file(name, data_vec);
    }
}

pub fn open(mut name: [u8; 20]) {
    let mut file_data = filesystem::read_file(name);
    infoln!("----- Starting BLIP -----");
    infoln!("[esc] - quit and save");
    infoln!("-------------------------");
    if name == [0; 20] {
        name[..7].copy_from_slice(b"unnamed")
    }
    render_background(name);
    let screen_buffer = update_text(&mut file_data);
    run(screen_buffer, name);
}