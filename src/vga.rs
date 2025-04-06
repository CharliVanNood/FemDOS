use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt::Write;
use x86_64::instructions::interrupts;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::vga::_warn(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::vga::_info(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! warnln {
    () => ($crate::warn!("\n"));
    ($($arg:tt)*) => ($crate::warn!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! infoln {
    () => ($crate::info!("\n"));
    ($($arg:tt)*) => ($crate::info!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _warn(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        let color = {
            WRITER.lock().back_color
        };
        WRITER.lock().set_color(4, color);
        WRITER.lock().write_fmt(args).unwrap();
        WRITER.lock().set_color(13, color);
    });
}

#[doc(hidden)]
pub fn _info(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        let color = {
            WRITER.lock().back_color
        };
        WRITER.lock().set_color(5, color);
        WRITER.lock().write_fmt(args).unwrap();
        WRITER.lock().set_color(13, color);
    });
}

pub fn get_color() -> u8 {
    WRITER.lock().back_color
}
pub fn set_color(foreground: u8, background: u8) {
    WRITER.lock().set_color(foreground, background);
}

pub fn remove_byte() {
    WRITER.lock().remove_byte();
}

pub fn set_header(time: (u8, u8, u8)) {
    WRITER.lock().set_header(time);
}

pub fn clear_screen() {
    for _ in 0..100 {
        println!("");
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new_color(Color::Pink, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        text_color: 13,
        back_color: 0
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new_color(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
    fn new_base(foreground: u8, background: u8) -> ColorCode {
        ColorCode(background << 4 | foreground)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    text_color: u8,
    back_color: u8,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    pub fn remove_byte(&mut self) {
        if self.column_position == 0 { return }
        self.column_position -= 1;

        let row: usize = BUFFER_HEIGHT - 1;
        let col = self.column_position;

        let color_code = self.color_code;
        self.buffer.chars[row][col].write(ScreenChar {
            ascii_character: 0,
            color_code,
        });
    }

    fn new_line(&mut self) {
        for row in 2..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }

        }
    }

    pub fn set_color(&mut self, foreground: u8, background: u8) {
        self.text_color = foreground;
        self.back_color = background;
        self.color_code = ColorCode::new_base(foreground, background);
    }

    pub fn set_header(&mut self, time: (u8, u8, u8)) {
        for i in 12..BUFFER_WIDTH {
            self.buffer.chars[0][i].write(ScreenChar {
                ascii_character: 0,
                color_code: ColorCode::new_color(Color::White, Color::Magenta)
            });
        }

        let hour_number_1 = time.0 / 10;
        let hour_number_2 = time.0 % 10;
        let min_number_1 = time.1 / 10;
        let min_number_2 = time.1 % 10;
        let sec_number_1 = time.2 / 10;
        let sec_number_2 = time.2 % 10;

        self.buffer.chars[0][0].write(ScreenChar {
            ascii_character: 0,
            color_code: ColorCode::new_color(Color::White, Color::White)
        });
        self.buffer.chars[0][1].write(ScreenChar {
            ascii_character: 0,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][2].write(ScreenChar {
            ascii_character: hour_number_1 + 48,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][3].write(ScreenChar {
            ascii_character: hour_number_2 + 48,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][4].write(ScreenChar {
            ascii_character: ':' as u8,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][5].write(ScreenChar {
            ascii_character: min_number_1 + 48,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][6].write(ScreenChar {
            ascii_character: min_number_2 + 48,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][7].write(ScreenChar {
            ascii_character: ':' as u8,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][8].write(ScreenChar {
            ascii_character: sec_number_1 + 48,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][9].write(ScreenChar {
            ascii_character: sec_number_2 + 48,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][10].write(ScreenChar {
            ascii_character: 0,
            color_code: ColorCode::new_color(Color::White, Color::Magenta)
        });
        self.buffer.chars[0][11].write(ScreenChar {
            ascii_character: 0,
            color_code: ColorCode::new_color(Color::White, Color::White)
        });
    }
}
