use crate::{alloc, applications::{assembly, basic, femc}, info, print, println, string::BigString, vec::{FileVec, Vec}, warnln};

use lazy_static::lazy_static;
use spin::Mutex;

pub struct FileSystem {
    files: FileVec,
    flow: i32
}
impl FileSystem {
    fn _get_item_name(&self, id: u32) -> [u8; 20] {
        self.files.iter()[id as usize].3
    }

    fn print_path(&self, id: u32) {
        let file = self.files.iter()[id as usize];
        if file.1 != -1 {
            self.print_path(file.1 as u32);
            info!("/");
        }
        for byte in file.3 {
            if byte == 0 { break; }
            info!("{}", byte as char);
        }
    }

    pub fn create_file(&mut self, parent: i32, filename: [u8; 20], filetype: u8, data: &str) {
        let data_bytes = data.bytes();

        let address = alloc::alloc(8192);

        let mut index = 0;
        for i in data_bytes {
            if address.0 + index > address.1 {
                break;
            }
            alloc::write_byte(address.0 + index, i as usize);
            index += 8;
        }

        self.files.add((self.files.len() as u32, parent, (address.0, address.1, index / 8), filename, filetype));

        print!("Created a new file with path ");
        info!("/");
        self.print_path(self.files.len() as u32 - 1);
        print!("\n");
    }

    pub fn set_flow(&mut self, flow: i32) {
        self.flow = flow;
    }

    // this being a list of 20 is the max amount of files that will be returned, why 20? sounds good to me tbh :3
    pub fn get_file_from_parent(&self, parent: i32) -> [(u32, i32, (usize, usize, usize), [u8; 20], u8); 20] {
        let mut files_returning = [(0, -1, (0, 1, 0), [1; 20], 0); 20];
        let mut files_returning_len = 0;
        for file in self.files.iter() {
            if file.1 == parent && files_returning_len < 20 {
                files_returning[files_returning_len] = file;
                files_returning_len += 1;
            }
        }
        files_returning
    }

    pub fn get_file_from_current_parent(&self) -> [(u32, i32, (usize, usize, usize), [u8; 20], u8); 20] {
        self.get_file_from_parent(self.flow)
    }
}

lazy_static! {
    pub static ref FILESYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem {
        files: FileVec::new(),
        flow: 1
    });
}

pub fn print_current_dir_files() {
    let files_found = FILESYSTEM.lock().get_file_from_current_parent();
    for file in files_found {
        if file.1 == -1 { continue; }
        for char_byte in file.3 {
            if char_byte == 0 { break; }
            print!("{}", char_byte as char);
        }
        print!("\n");
    }
}

pub fn change_flow(name: [u8; 20]) {
    let files = {
        FILESYSTEM.lock().get_file_from_current_parent()
    };
    for file in files {
        if file.3 == name {
            FILESYSTEM.lock().set_flow(file.0 as i32);
        }
    }
}

pub fn find_file(name: [u8; 20]) -> (u32, i32, (usize, usize, usize), [u8; 20], u8) {
    let files = {
        FILESYSTEM.lock().get_file_from_current_parent()
    };
    for file in files {
        let mut file_name: [u8; 20] = [0; 20];
        for byte in file.3.iter().enumerate() {
            if *byte.1 == 61 { break; }
            file_name[byte.0] = *byte.1;
        }
        if name == file_name {
            return file
        }
    }
    println!("This file doesn't seem to exist :c");
    return (0, 0, (0, 0, 0), [0; 20], 0)
}

pub fn create_file(parent: i32, filename: &str, filetype: &str, data: &str) {
    let mut filename_bytes = [0; 20];
    let mut filename_bytes_len = 0;

    let file_type = {
        match filetype {
            "a" => 1,   // assembly
            "b" => 2,   // basic
            "fc" => 3,  // FemC
            _ => 0      // Flow
        }
    };

    let is_flow = filetype.bytes().len() == 0;

    let filename_parsed = filename.bytes();
    for byte in filename_parsed {
        filename_bytes[filename_bytes_len] = byte;
        filename_bytes_len += 1;
    }

    if !is_flow {
        filename_bytes[filename_bytes_len] = 61;
        filename_bytes_len += 1;
        let filetype_parsed = filetype.bytes();
        for byte in filetype_parsed {
            filename_bytes[filename_bytes_len] = byte;
            filename_bytes_len += 1;
        }
    }

    FILESYSTEM.lock().create_file(parent, filename_bytes, file_type, data);
}

pub fn read_file(name: [u8; 20]) {
    let file = find_file(name);
    let file_start = file.2.0;
    let file_size = file.2.2;

    for i in 0..file_size {
        let byte = alloc::read_byte(file_start + i * 8) as u8;
        print!("{}", byte as char);
    }
    print!("\n");
}

pub fn read_image(name: [u8; 20]) -> Vec {
    let file = find_file(name);
    let file_start = file.2.0;
    let file_size = file.2.2;

    let mut contents = Vec::new();

    for i in 0..file_size {
        let byte = alloc::read_byte(file_start + i * 8);
        contents.add(byte);
    }

    return contents
}

pub fn run_file(name: [u8; 20]) {
    let file = find_file(name);
    let file_start = file.2.0;
    let file_size = file.2.2;
    let file_type = file.4;

    match file_type {
        1 => {
            let mut file_data: BigString = BigString::new();
            for i in 0..file_size {
                let byte = alloc::read_byte(file_start + i * 8) as u8;
                file_data.add(byte);
            }
            assembly::exec(file_data);
        }
        2 => {
            let mut file_data: [u8; 512] = [0; 512];
            for i in 0..file_size {
                if i >= 512 {
                    break;
                }
                let byte = alloc::read_byte(file_start + i * 8) as u8;
                file_data[i] = byte;
            }
            basic::exec(file_data);
        }
        3 => {
            let mut file_data: [u8; 256] = [0; 256];
            for i in 0..file_size {
                if i >= 256 {
                    break;
                }
                let byte = alloc::read_byte(file_start + i * 8) as u8;
                file_data[i] = byte;
            }
            femc::exec(file_data);
        }
        _ => warnln!("Unrecognized file type, I can't run this :C")
    }
}

pub fn install_base_os() {
    println!("Installing FemDOS");
    create_file(1, "file1", "b", "PRINT \"Hello,world\"");
    create_file(1, "python1", "fc", "print 1 + 10 * 10\nprint 10 + 10\ntest = 10\ntest2 = 20\nprint test\nprint test2");
    create_file(1, "loop_test_1", "fc", "do\nprint 10\nrepeat 10");
    create_file(1, "loop_test_2", "fc", "do\nprint 10\nrepeat 0\ndo\nprint 5\nrepeat 10");
    create_file(1, "if_test_1", "fc", "if 10 == 10\nprint 10\nend\nif 10 == 5\nprint 5\nend");
    create_file(1, "color_test_1", "fc", "color 1 1");
    create_file(1, "color_test_2", "fc", "color 11 11\nprint true\ncolor 13 13\nprint true\ncolor 15 15\nprint true\ncolor 13 13\nprint true\ncolor 11 11\nprint true\ncolor 15 0");

    create_file(1, "basic", "b", "
    PRINT \"loops\"
    PRINT \"are\"
    PRINT \"amazing\"

    DO
    PRINT TRUE
    PRINT FALSE
    LOOP
    PRINT TRUE
    ");

    create_file(1, "asm", "a", "
    section .data
    hello db \"Hello,World!\", 0
    section .text
    global _start
    _start:

    mov eax, 4
    mov ebx, 1
    mov ecx, hello
    mov edx, 13
    int 0x80

    mov eax, 1
    xor ebx, ebx
    int 0x80
    ");

    create_file(1, "smiley", "img", "6;6;255255255255255255255255255255255255255255255255255255255255255000000000000000000000000000255255255000000000255255255255255255255255255000000000255255255255255255255255255255255255255255255255255255000000000255255255000000000255255255255255255000000000255255255000000000255255255");
}