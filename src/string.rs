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

    // Create an empty string, pass if no string will be created, saves memory
    #[allow(dead_code)]
    pub fn empty() -> Self {
        let heap_start = alloc::alloc(0);
        Self {
            size: 0,
            heap_start: heap_start.0,
            heap_size: heap_start.1 - heap_start.0,
            heap_end: heap_start.1
        }
    }

    // Convert a &str to a Big String
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

    // Converts 64 bytes into a big string (wastefull 9/10 times)
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

    // Convert 256 bytes into a big string
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

    // Convert 512 bytes into a big string
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

    // Add character to the end of the string
    #[allow(dead_code)]
    pub fn add(&mut self, value: u8) {
        if self.size >= self.heap_size {
            warnln!("Reached String limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value as usize);
        self.size += 8;
    }

    // Get a character from this string at a certain index
    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> usize {
        if address * 8 >= self.size {
            warnln!("Address out of range :c");
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    // Set a character in this string, also at a certain index
    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, value: usize) {
        if address * 8 < self.size {
            alloc::write_byte(self.heap_start + address * 8, value);
        }
    }

    // This is a somewhat strange function, it adds at a certain index but if it's out of range, it padds with 0's
    #[allow(dead_code)]
    pub fn set_add(&mut self, address: usize, value: usize) {
        if address * 8 >= self.size {
            self.size = (address + 1) * 8;
        }
        alloc::write_byte(self.heap_start + address * 8, value);
    }

    // Get the length of this string
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }

    // Prints the current string :D
    #[allow(dead_code)]
    pub fn print(&self) {
        for index in 0..self.len() {
            print!("{}", self.get(index) as u8 as char);
        }
        print!("\n");
    }

    // Check if this string includes a certain word and get the first index, mirrors javascript
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

    // Replace a certain word in this string by a set value
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

    // Unallocate the current vector (Don't forget, expecially for such a big string)
    #[allow(dead_code)]
    pub fn remove(&self) {
        alloc::unalloc(self.heap_start, self.heap_size);
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

    // Convert &str to a valid String
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

    // Convert a list of 64 bytes to a string
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

    // Convert a list of 256 bytes to a string
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

    // Add a character at the end and increment the size of this string :D
    #[allow(dead_code)]
    pub fn add(&mut self, value: u8) {
        if self.size >= self.heap_size {
            warnln!("Reached String limit :c");
            return;
        }
        alloc::write_byte(self.heap_start + self.size, value as usize);
        self.size += 8;
    }

    // Get a specific character
    #[allow(dead_code)]
    pub fn get(&self, address: usize) -> usize {
        if address * 8 >= self.size {
            warnln!("Address out of range for {} :c", address);
            return 0;
        }
        alloc::read_byte(self.heap_start + address * 8)
    }

    // Set a character in the current string
    #[allow(dead_code)]
    pub fn set(&mut self, address: usize, value: usize) {
        if address * 8 < self.size {
            alloc::write_byte(self.heap_start + address * 8, value);
        }
    }

    // Get the length of this string
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.size / 8
    }

    // Print the current string
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

    // This function gets the first index of a certain word
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

    // This function is used to replace a word in the current string by a value
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

    // This function is used to deallocate, don't forget to please
    #[allow(dead_code)]
    pub fn remove(&self) {
        alloc::unalloc(self.heap_start, self.heap_size);
    }
}