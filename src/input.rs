use crate::alloc;
use crate::clock;
use crate::window;
use crate::{print, println, warnln};
use crate::vga;
use crate::applications;
use crate::filesystem;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref CURRENT_TEXT: Mutex<[u8; 256]> = Mutex::new([0; 256]);
    static ref CURRENT_TEXT_END: Mutex<usize> = Mutex::new(0);
    pub static ref KEYPRESSES: Mutex<([u8; 8], u8)> = Mutex::new(([0; 8], 0));
}

#[allow(dead_code)]
pub fn check_events() {
    let time = clock::get_time();
    vga::set_header(time);

    let keypresses = {
        let lock = KEYPRESSES.lock();
        lock.clone()
    };

    for keypress in keypresses.0 {
        if keypress == 0 { break; }
        add_key(keypress);
    }

    KEYPRESSES.lock().0 = [0; 8];
    KEYPRESSES.lock().1 = 0;
}

#[allow(dead_code)]
pub fn set_text(characters: [u8; 256]) {
    let mut text = CURRENT_TEXT.lock();
    *text = characters;
}

#[allow(dead_code)]
pub fn get_text() -> [u8; 256] {
    let text = CURRENT_TEXT.lock();
    *text
}

#[allow(dead_code)]
pub fn add_key(character: u8) {
    match character {
        10 => {
            match_commands();
            return;
        },
        8 => {
            remove_byte();
            return;
        }
        _ => {}
    }

    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end < 255 {
        text[*text_end] = character;
        *text_end += 1;
        print!("{}", character as char);
    }
}

fn remove_byte() {
    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end > 0 {
        *text_end -= 1;
        text[*text_end] = 0;
        vga::remove_byte();
    }
}

fn print_help_command() {
    println!("\nWe have these general commands");
    println!("   [ping]             - Just a simple test command");
    println!("   [femc] [code]      - Run femc commands");
    println!("   [basic] [code]     - Run BASIC commands");
    println!("   [color]            - Toggle the background color");
    println!("   [clear]            - Clear the screen");
    println!("   [fl]               - Show the items in the current flow");
    println!("   [go] [flow name]   - Change to a different flow");
    println!("   [pong]             - The game pong");
    println!("   [cat]              - Read a file");
    println!("   [time]             - Time will show you the current time according to bios");
    println!("   [timeset] [hour]   - Timeset will set the current hour");
    println!("   [per]              - Performance will show you system details");
    println!("   [run] [file name]  - Run runs the actual files\n");
}

#[allow(dead_code)]
pub fn match_commands() {
    let commands = [
        "info", "ping", "color", "clear", "help", "femc", "fl", "go", 
        "install", "pong", "cat", "run", "per", "time", "input", "timeset",
        "basic", "screen"
        ];

    print!("\n");

    let mut command_processed = false;
    for command in commands {
        let command_bytes = command.bytes();
        let command_length = command_bytes.len();
        let command_written = get_text();
        let mut is_command = true;

        let mut i = 0;
        for byte in command_bytes {
            if i + 1 == command_length && command_written[i + 1] != 0 && command_written[i + 1] != 32 {
                is_command = false;
            }
            if byte != command_written[i] as u8 {
                is_command = false;
            }
            i += 1;
        }

        if is_command {
            command_processed = true;
            match command {
                "info" => print_help_command(),
                "help" => print_help_command(),
                "ping" => println!("Pong"),
                "color" => {
                    print!("Changed the color to black");
                    let color = vga::get_color();
                    if color == 15 {
                        vga::set_color(13, 0);
                    } else {
                        vga::set_color(13, 15);
                    }
                    print!("\n");
                },
                "clear" => {
                    window::clear_screen();
                    print!("The screen has been cleared");
                    print!("\n");
                },
                "femc" => applications::femc::exec(command_written),
                "basic" => {
                    let mut command_written_512 = [0u8; 512];
                    command_written_512[..256].copy_from_slice(&command_written);
                    applications::basic::exec(command_written_512)
                },
                "fl" => filesystem::print_current_dir_files(),
                "go" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 3..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    filesystem::change_flow(name);
                },
                "install" => filesystem::install_base_os(),
                "pong" => applications::pong::play(),
                "cat" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 4..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    filesystem::read_file(name);
                },
                "run" => {
                    let mut name = [0; 20];
                    let mut name_len = 0;

                    for byte_index in 4..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        name[name_len] = byte as u8;
                        name_len += 1;
                    }

                    filesystem::run_file(name);
                },
                "per" => {
                    let ram_usage = alloc::get_usage();
                    println!("\n   Ram: {:.2}%", (ram_usage.0 as f32 / ram_usage.1 as f32) * 100.0);
                    println!("   {} Bytes / {} Bytes", ram_usage.0, ram_usage.1);
                    println!("   {} KB / {} KB", ram_usage.0 / 1000, ram_usage.1 / 1000);
                    println!("   {} MB / {} MB\n", ram_usage.0 / 1000000, ram_usage.1 / 1000000);
                    println!("   Disk: 0%\n");
                },
                "time" => {
                    clock::print_time();
                },
                "timeset" => {
                    let mut time = [0; 3];
                    let mut time_len = 0;

                    for byte_index in 8..23 {
                        let byte = command_written[byte_index];
                        if byte == 0 { break; }
                        time[time_len] = byte as u8;
                        time_len += 1;
                    }

                    let mut time_number = 0;

                    for byte in time.iter().enumerate() {
                        if *byte.1 == 0 { break; }
                        let byte_number = *byte.1 as i32 - 48;
                        time_number += byte_number * 10_i32.pow((time_len - byte.0 - 1) as u32);
                    }

                    clock::set_time(time_number as u8);
                },
                "input" => println!("neh"),
                _ => warnln!("This command is unimplemented :C")
            }
        }
    }
    if !command_processed {
        warnln!("This command does not seem to exist :C");
    }

    print!("-> ");

    {
        let mut text = CURRENT_TEXT.lock();
        let mut text_end = CURRENT_TEXT_END.lock();
        *text = [0; 256];
        *text_end = 0;
    }
}