use crate::alloc;
use crate::clock::print_time;
use crate::{print, println, warnln};
use crate::vga;
use crate::applications;
use crate::filesystem;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref CURRENT_TEXT: Mutex<[u8; 256]> = Mutex::new([0; 256]);
    static ref CURRENT_TEXT_END: Mutex<usize> = Mutex::new(0);
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
pub fn add_key(character: u8) -> bool {
    match character {
        10 => {
            match_commands();
            return false;
        },
        8 => {
            remove_byte();
            return false;
        }
        _ => {}
    }

    let mut text = CURRENT_TEXT.lock();
    let mut text_end = CURRENT_TEXT_END.lock();
    
    if *text_end < 255 {
        text[*text_end] = character;
        *text_end += 1;
        true
    } else {
        println!("You're at the typing limit :c");
        false
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
    println!("   [color]            - Toggle the background color");
    println!("   [clear]            - Clear the screen");
    println!("   [fl]               - Show the items in the current flow");
    println!("   [go] [flow name]   - Change to a different flow");
    println!("   [pong]             - The game pong");
    println!("   [cat]              - Read a file");
    println!("   [time]             - Time will show you the current time according to bios");
    println!("   [per]              - Performance will show you system details\n");
}

#[allow(dead_code)]
pub fn match_commands() {
    let commands = ["info", "ping", "color", "clear", "help", "femc", "fl", "go", "install", "pong", "cat", "run", "per", "time"];

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
                    vga::clear_screen();
                    print!("The screen has been cleared");
                    print!("\n");
                },
                "femc" => applications::femc::exec(command_written),
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
                },
                "time" => {
                    print_time();
                },
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