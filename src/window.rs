use volatile::Volatile;

use crate::renderer::text::CHARACTERS;

const BUFFER_WIDTH: usize = 320;
const BUFFER_HEIGHT: usize = 200;

#[repr(transparent)]
struct Buffer {
    pixels: [Volatile<u8>; BUFFER_WIDTH * BUFFER_HEIGHT],
}

// yes I did do this myself, I know there might be a lookup table out there, but I decided to take the tedious route, why else would I make an OS
const COLOR_PALETTE: [(u8, u8, u8); 216] = [
    (0, 0, 0),
    (0, 0, 170),
    (0, 170, 0),
    (0, 170, 170),
    (170, 0, 0),
    (170, 0, 170),
    (170, 87, 0),
    (170, 170, 170),
    (87, 87, 87),
    (87, 87, 255),
    (87, 255, 87),
    (255, 87, 87),
    (255, 87, 255),
    (255, 255, 87),
    (255, 255, 255),
    (0, 0, 0),
    (23, 23, 23),
    (32, 32, 32),
    (47, 47, 47),
    (56, 56, 56),
    (71, 71, 71),
    (80, 80, 80),
    (96, 96, 96),
    (112, 112, 112),
    (128, 128, 128),
    (144, 144, 144),
    (160, 160, 160),
    (183, 183, 183),
    (200, 200, 200),
    (224, 224, 224),
    (255, 255, 255),
    (0, 0, 255),
    (64, 0, 255),
    (127, 0, 255),
    (191, 0, 255),
    (255, 0, 255),
    (255, 0, 191),
    (255, 0, 127),
    (255, 0, 64),
    (255, 0, 0),
    (255, 64, 0),
    (255, 127, 0),
    (255, 191, 0),
    (255, 255, 0),
    (191, 255, 0),
    (127, 255, 0),
    (64, 255, 0),
    (0, 255, 0),
    (0, 255, 64),
    (0, 255, 127),
    (0, 255, 191),
    (0, 255, 255),
    (0, 191, 255),
    (0, 127, 255),
    (0, 64, 255),
    (127, 127, 255),
    (159, 127, 255),
    (191, 127, 255),
    (223, 127, 255),
    (255, 127, 255),
    (255, 127, 223),
    (255, 127, 191),
    (255, 127, 159),
    (255, 127, 127),
    (255, 159, 127),
    (255, 191, 127),
    (255, 223, 127),
    (255, 255, 127),
    (223, 255, 127),
    (191, 255, 127),
    (159, 255, 127),
    (127, 255, 127),
    (127, 255, 159),
    (127, 255, 191),
    (127, 255, 223),
    (127, 255, 255),
    (127, 223, 255),
    (127, 191, 255),
    (127, 159, 255),
    (183, 183, 255),
    (199, 183, 255),
    (216, 183, 255),
    (232, 183, 255),
    (255, 183, 255),
    (255, 183, 232),
    (255, 183, 216),
    (255, 183, 199),
    (255, 183, 183),
    (255, 199, 183),
    (255, 216, 183),
    (255, 232, 183),
    (255, 255, 183),
    (232, 255, 183),
    (216, 255, 183),
    (199, 255, 183),
    (183, 255, 183),
    (183, 255, 199),
    (183, 255, 216),
    (183, 255, 232),
    (183, 255, 255),
    (183, 232, 255),
    (183, 216, 255),
    (183, 199, 255),
    (0, 0, 112),
    (31, 0, 112),
    (56, 0, 112),
    (87, 0, 112),
    (112, 0, 112),
    (112, 0, 87),
    (112, 0, 56),
    (112, 0, 31),
    (112, 0, 0),
    (112, 31, 0),
    (112, 56, 0),
    (112, 87, 0),
    (112, 112, 0),
    (87, 112, 0),
    (56, 112, 0),
    (31, 112, 0),
    (0, 112, 0),
    (0, 112, 31),
    (0, 112, 56),
    (0, 112, 87),
    (0, 112, 112),
    (0, 87, 112),
    (0, 56, 112),
    (0, 31, 112),
    (56, 56, 112),
    (71, 56, 112),
    (87, 56, 112),
    (96, 56, 112),
    (112, 56, 112),
    (112, 56, 96),
    (112, 56, 87),
    (112, 56, 71),
    (112, 56, 56),
    (112, 71, 56),
    (112, 87, 56),
    (112, 96, 56),
    (112, 112, 56),
    (96, 112, 56),
    (87, 112, 56),
    (71, 112, 56),
    (56, 112, 56),
    (56, 112, 71),
    (56, 112, 87),
    (56, 112, 96),
    (56, 112, 112),
    (56, 96, 112),
    (56, 87, 112),
    (56, 71, 112),
    (80, 80, 112),
    (88, 80, 112),
    (96, 80, 112),
    (104, 80, 112),
    (112, 80, 112),
    (112, 80, 104),
    (112, 80, 96),
    (112, 80, 88),
    (112, 80, 80),
    (112, 88, 80),
    (112, 96, 80),
    (112, 104, 80),
    (112, 112, 80),
    (104, 112, 80),
    (96, 112, 80),
    (88, 112, 80),
    (80, 112, 80),
    (80, 112, 88),
    (80, 112, 96),
    (80, 112, 104),
    (80, 112, 112),
    (80, 104, 112),
    (80, 96, 112),
    (80, 88, 112),
    (0, 0, 64),
    (16, 0, 64),
    (32, 0, 64),
    (48, 0, 64),
    (64, 0, 64),
    (64, 0, 48),
    (64, 0, 32),
    (64, 0, 16),
    (64, 0, 0),
    (64, 16, 0),
    (64, 32, 0),
    (64, 48, 0),
    (64, 64, 0),
    (48, 64, 0),
    (32, 64, 0),
    (16, 64, 0),
    (0, 64, 0),
    (0, 64, 16),
    (0, 64, 32),
    (0, 64, 48),
    (0, 64, 64),
    (0, 48, 64),
    (0, 32, 64),
    (0, 16, 64),
    (32, 32, 64),
    (40, 32, 64),
    (48, 32, 64),
    (56, 32, 64),
    (64, 32, 64),
    (64, 32, 56),
    (64, 32, 48),
    (64, 32, 40),
    (64, 32, 32),
    (64, 40, 32),
    (64, 48, 32),
    (64, 56, 32),
    (64, 64, 32),
    (56, 64, 32),
    (48, 64, 32),
    (40, 64, 32),
    (32, 64, 32),
];

fn get_next_color() -> u8 {
    COLOR_PALETTE.len() as u8
}

#[allow(dead_code)]
fn get_rgb(r: u8, g: u8, b: u8) -> u8 {
    let mut closest_color: (i16, usize) = (-1, 999999);

    for color in COLOR_PALETTE.iter().enumerate() {
        let dr = r as isize - color.1.0 as isize;
        let dg = g as isize - color.1.1 as isize;
        let db = b as isize - color.1.2 as isize;
        let color_distance = (dr * dr + dg * dg + db * db) as usize;

        if color_distance < closest_color.1 {
            closest_color = (color.0 as i16, color_distance);
            if color_distance < 100 {
                return closest_color.0 as u8
            }
        }
    }

    closest_color.0 as u8
}

fn get_pixel_index(x: usize, y: usize) -> usize {
    x + y * BUFFER_WIDTH
}

fn draw_character(buffer: &mut Buffer, character: u8, x: usize, y: usize) {
    let characters = CHARACTERS[character as usize];

    for char in characters.iter().enumerate() {
        if char.1 == &true {
            buffer.pixels[get_pixel_index(x + char.0 % 5, y + char.0 / 5)].write(15);
        } else {
            buffer.pixels[get_pixel_index(x + char.0 % 5, y + char.0 / 5)].write(0);
        }
    }
}

fn clear_characters(buffer: &mut Buffer, terminal_buffer: &mut [[u8; 25]; 19], line: usize) {
    for i in 0..24 {
        draw_character(buffer, 0, 9 + i * 6, 183 - 10 * line);
        terminal_buffer[line][i] = 0;
    }
}

fn shift_characters(buffer: &mut Buffer, terminal_buffer: &mut [[u8; 25]; 19]) {
    for line in 1..19 {
        for i in 0..24 {
            let character = terminal_buffer[18 - line][i];
            terminal_buffer[19 - line][i] = character;
            draw_character(buffer, character, 9 + i * 6, 183 - 10 * (19 - line));
            //draw_character(buffer, 100, 9 + i * 6, 183 - 10 * (line + 1));
        }
    }
    clear_characters(buffer, terminal_buffer, 0);
}

pub fn init() {
    let buffer: &mut Buffer = unsafe { &mut *(0xa0000 as *mut Buffer) };
    
    let _frames = [(0, 0, 160, 100); 4];

    for x in 0..BUFFER_WIDTH {
        for y in 0..BUFFER_HEIGHT {
            if x > 160 {
                buffer.pixels[get_pixel_index(x, y)].write(get_next_color());
            } else {
                buffer.pixels[get_pixel_index(x, y)].write(0);
            }
        }
    }

    let terminal_line = "hello world\nthis is a line of text\ngood day yall\nblack jack is overrated\ni will give you a medal\npot dor dot\ni have a question\nzen browser";

    let mut terminal_column_position = 0;
    let mut terminal_buffer: [[u8; 25]; 19] = [[0; 25]; 19];

    for char in terminal_line.bytes() {
        let mut char_writing = char;
        if char == b'\n' {
            shift_characters(buffer, &mut terminal_buffer);
            terminal_column_position = 0;
            continue;
        }
        if terminal_column_position == 24 {
            shift_characters(buffer, &mut terminal_buffer);
            terminal_column_position = 0;
        }
        if char_writing >= CHARACTERS.len() as u8 {
            char_writing = 0;
        }
        draw_character(buffer, char_writing,  9 + terminal_column_position * 6, 183);
        terminal_buffer[0][terminal_column_position] = char_writing;
        terminal_column_position += 1;
    }
}
