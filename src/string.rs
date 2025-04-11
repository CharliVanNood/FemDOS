use core::usize;

use crate::{print, warnln};
use crate::alloc;

#[allow(dead_code)]
pub struct BigString {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl BigString {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(8192);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from(value: &str) -> Self {
        let heap_start = alloc::alloc(8192);
        let mut size = 0;

        for byte in value.bytes() {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from_b64(value: [u8; 64]) -> Self {
        let heap_start = alloc::alloc(512);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from_b256(value: [u8; 256]) -> Self {
        let heap_start = alloc::alloc(2048);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from_b512(value: [u8; 512]) -> Self {
        let heap_start = alloc::alloc(4096);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, value: u8) {
        if self.size >= self.heap_size {
            warnln!("Reached String limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value as usize);
        self.size += 8;
    }

    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> usize {
        if address * 8 >= self.size {
            warnln!("Address out of range :c");
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, value: usize) {
        if address * 8 < self.size {
            alloc::write_byte(self.heap_start + address * 8, value);
        }
    }

    #[allow(dead_code)]
    pub fn set_add(&mut self, address: usize, value: usize) {
        if address * 8 >= self.size {
            self.size = (address + 1) * 8;
        }
        alloc::write_byte(self.heap_start + address * 8, value);
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for index in 0..self.len() {
            if self.get(index) as u8 as char == '\n' {
                print!(":new:");
                continue;
            }
            print!("{}", self.get(index) as u8 as char);
        }
        print!("\n");
    }

    #[allow(dead_code)]
    pub fn includes(&self, needle: &str) -> i32 {
        let first_character = needle.bytes().next().unwrap();
        let needle_length = needle.bytes().len();

        for index in 0..self.len() {
            let character = self.get(index) as u8;
            if character == first_character {
                if needle_length == 1 {
                    return index as i32;
                } else {
                    let mut matches = true;
                    let mut offset = 0;
                    for character in needle.bytes() {
                        let character_self = self.get(index + offset) as u8;
                        if character != character_self {
                            matches = false;
                        }
                        offset += 1;
                    }
                    if matches {
                        return index as i32;
                    }
                }
            }
        }
        return -1;
    }

    pub fn replace(&mut self, needle: &str, value: &str)  {
        let needle_index = self.includes(needle);
        if needle_index == -1 { return; }

        let needle_end = needle_index as usize + needle.bytes().len();
        let offset = value.bytes().len() as i16 - needle.bytes().len() as i16;

        self.size += offset as usize * 8;

        for moving in 0..self.len() - needle_end {
            if self.size as i16 / 8 - moving as i16 - offset < 0 {
                continue;
            }
            self.set(self.len() - moving, self.get((self.len() as i16 - moving as i16 - offset) as usize));
        }
        for byte in value.bytes().enumerate() {
            self.set(needle_index as usize + byte.0, byte.1 as usize);
        }
    }
}

#[allow(dead_code)]
pub struct String {
    size: usize,
    heap_start: usize,
    heap_size: usize,
    heap_end: usize
}
impl String {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let heap_start = alloc::alloc(512);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from(value: &str) -> Self {
        let heap_start = alloc::alloc(512);
        let mut size = 0;

        for byte in value.bytes() {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from_b64(value: [u8; 64]) -> Self {
        let heap_start = alloc::alloc(2048);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn from_b256(value: [u8; 256]) -> Self {
        let heap_start = alloc::alloc(2048);
        let mut size = 0;

        for byte in value {
            if size >= heap_start.1 - heap_start.0 {
                warnln!("Reached String limit :c");
                continue;
            }
            alloc::write_byte(heap_start.0 + size, byte as usize);
            size += 8;
        }

        Self {
            size: size,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, value: u8) {
        if self.size >= self.heap_size {
            warnln!("Reached String limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value as usize);
        self.size += 8;
    }

    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> usize {
        if address * 8 >= self.size {
            warnln!("Address out of range for {} :c", address);
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, value: usize) {
        if address * 8 < self.size {
            alloc::write_byte(self.heap_start + address * 8, value);
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for index in 0..self.len() {
            if self.get(index) as u8 as char == '\n' {
                print!(":new:");
                continue;
            }
            print!("{}", self.get(index) as u8 as char);
        }
        print!("\n");
    }

    #[allow(dead_code)]
    pub fn includes(&self, needle: &str) -> i32 {
        let first_character = needle.bytes().next().unwrap();
        let needle_length = needle.bytes().len();

        for index in 0..self.len() {
            let character = self.get(index) as u8;
            if character == first_character {
                if needle_length == 1 {
                    return index as i32;
                } else {
                    let mut matches = true;
                    let mut offset = 0;
                    for character in needle.bytes() {
                        let character_self = self.get(index + offset) as u8;
                        if character != character_self {
                            matches = false;
                        }
                        offset += 1;
                    }
                    if matches {
                        return index as i32;
                    }
                }
            }
        }
        return -1;
    }

    #[allow(dead_code)]
    pub fn replace(&mut self, needle: &str, value: &str)  {
        let needle_index = self.includes(needle);
        if needle_index == -1 { return; }

        let needle_end = needle_index as usize + needle.bytes().len();
        let offset = value.bytes().len() as i16 - needle.bytes().len() as i16;

        for moving in 0..self.len() - needle_end {
            if self.size as i16 / 8 - moving as i16 - offset < 0 {
                continue;
            }
            self.set(self.size / 8 - moving, self.get((self.size as i16 / 8 - moving as i16 - offset) as usize));
        }
        for byte in value.bytes().enumerate() {
            self.set(needle_index as usize + byte.0, byte.1 as usize);
        }
    }
}