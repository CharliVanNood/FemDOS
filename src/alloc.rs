use core::ptr;
use bootloader::{bootinfo, BootInfo};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{println, warnln, infoln};
use crate::alloc::bootinfo::MemoryRegionType;

pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    used: [(usize, usize, bool); 512]
}

impl Allocator {
    pub fn new(heap_start: usize, heap_size: usize) -> Self {
        let mut used = [(0, 0, false); 512];
        used[0] = (heap_start, heap_start + heap_size, true);

        Self {
            heap_start,
            heap_end: heap_start + heap_size,
            next: heap_start,
            used: used
        }
    }

    fn _print_regions(&self) {
        for section_printing in self.used {
            if section_printing == (0, 0, false) { break; }
            if section_printing.2 {
                println!("available: {} {}", section_printing.0 - self.heap_start, section_printing.1 - self.heap_start);
            } else {
                warnln!("reserved: {} {}", section_printing.0 - self.heap_start, section_printing.1 - self.heap_start);
            }
        }
    }

    fn section_exists(&self, index: usize) -> bool {
        if index > self.used.len() {
            warnln!("Section out of range!");
            return false;
        }
        if self.used[index] == (0, 0, false) {
            warnln!("Section {} uninitialized!", index);
            return false;
        }
        return true;
    }

    fn split_section(&mut self, index: usize) {
        if !self.section_exists(index) { return; }

        let section = self.used[index];
        let section_size = section.1 - section.0;
        let section_size_new = section_size / 2 - ((section_size / 2) % 8);

        self.used[index].1 = section.0 + section_size_new;

        for section_new_index in 0..self.used.len() {
            if self.used[section_new_index] == (0, 0, false) {
                self.used[section_new_index] = (section.0 + section_size_new, section_size + section.0, true);
                break;
            }
        }
    }

    fn reserve_section(&mut self, index: usize, size: usize) -> (usize, usize) {
        if !self.section_exists(index) { return (0, 0); }

        let section = self.used[index];
        let section_size = section.1 - section.0;

        self.used[index].1 = section.0 + size;
        self.used[index].2 = false;
        self.next += size;

        for section_new_index in 0..self.used.len() {
            if self.used[section_new_index] == (0, 0, false) {
                self.used[section_new_index] = (section.0 + size, section_size + section.0, true);
                break;
            }
        }

        //self.print_regions();

        (self.used[index].0, self.used[index].0 + self.used[index].1)
    }

    fn get_largest_section(&self) -> (usize, usize) {
        let mut largest_section = (0, 0);

        for section in self.used.iter().enumerate() {
            if section.1 == &(0, 0, false) { break; }
            let section_size = section.1.1 - section.1.0;
            if section_size > largest_section.1 && section.1.2 {
                largest_section = (section.0, section_size);
            }
        }

        largest_section
    }

    fn set_heap(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;

        let mut used = [(0, 0, false); 512];
        used[0] = (heap_start, heap_start + heap_size, true);

        self.used = used;
    }

    pub fn alloc(&mut self, size_raw: usize) -> (usize, usize) {
        let size = size_raw * 8;
        if self.next + size > self.heap_end {
            warnln!("Address 0x{:x} is out of range", self.next + size);
            return (0, 0);
        }

        let largest_section = self.get_largest_section();

        let mut needs_splitting = false;
        if largest_section.0 > 0 && self.used[largest_section.0 - 1].2 == false {
            needs_splitting = true;
        }

        if size > largest_section.1 {
            warnln!("No more sectors available");
            return (0, 0);
        }

        if needs_splitting {
            self.split_section(largest_section.0);
        }
        self.reserve_section(largest_section.0, size)
    }

    pub fn unalloc(&mut self, heap_start: usize, heap_end: usize) {
        for section_index in 0..self.used.len() {
            let section = self.used[section_index];
            if (section.0, section.1) == (heap_start, heap_end) {
                self.used[section_index].2 = true;
                self.next -= heap_end - heap_start;
            }
        }
    }
}

lazy_static! {
    pub static ref ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new(0, 0));
}

pub fn get_usage() -> (usize, usize) {
    let next = { ALLOCATOR.lock().next };
    let heap_start = { ALLOCATOR.lock().heap_start };
    let heap_end = { ALLOCATOR.lock().heap_end };
    (next - heap_start, heap_end - heap_start)
}

pub fn set_heap(heap_start: usize, heap_size: usize) {
    ALLOCATOR.lock().set_heap(heap_start, heap_size);
}

pub fn alloc(size: usize) -> (usize, usize) {
    if size == 0 { return (0, 0) }
    ALLOCATOR.lock().alloc(size)
}

pub fn unalloc(address: usize, size: usize) {
    ALLOCATOR.lock().unalloc(address, size);
}

pub fn write_byte(address: usize, value: usize) {
    unsafe {
        let heap_end = { ALLOCATOR.lock().heap_end };
        if address > heap_end {
            warnln!("Address 0x{:x} is out of range! :C", address);
        } else {
            ptr::write((address) as *mut usize, value);
        }
    }
}

pub fn read_byte(address: usize) -> usize {
    unsafe {
        let heap_end = { ALLOCATOR.lock().heap_end };
        if address > heap_end {
            warnln!("Address 0x{:x} is out of range! :C", address);
        } else {
            return ptr::read((address) as *mut usize);
        }
    }

    0
}

pub fn _ram_test(address: usize, length: usize) {
    for byte in 0..length {
        unsafe {
            if byte == 512 || byte > 516 {
                continue;
            }
            println!("reading {} at byte index {}", address + byte * 8, byte);
            ptr::write((address + byte * 8) as *mut usize, 255);
            let byte_read = ptr::read((address + byte * 8) as *mut usize);
            ptr::write((address + byte * 8) as *mut usize, 0);
            if byte_read != 255 {
                warnln!("BYTE {} DIDN'T READ PROPERLY!!! :CC", address + byte * 8);
                return;
            }
        }
    }
}

pub fn _memory_regions(boot_info: &'static BootInfo) {
    for item in boot_info.memory_map.iter() {
        let range = item.range;
        let start = range.start_addr();
        let end = range.end_addr();
        
        match item.region_type {
            MemoryRegionType::Usable => infoln!("This memory is usable {} to {}", start, end),
            MemoryRegionType::Reserved => warnln!("This memory is reserved {} to {}", start, end),
            MemoryRegionType::InUse => warnln!("This memory is in use {} to {}", start, end),
            MemoryRegionType::BadMemory => warnln!("This memory is bad {} to {}", start, end),
            MemoryRegionType::PageTable => infoln!("This is the page table {} to {}", start, end),
            MemoryRegionType::Bootloader => infoln!("This is the boot loader {} to {}", start, end),
            MemoryRegionType::Empty => infoln!("This memory is empty {} to {}", start, end),
            MemoryRegionType::Kernel => infoln!("This is the kernel {} to {}", start, end),
            MemoryRegionType::BootInfo => infoln!("This is boot info {} to {}", start, end),
            MemoryRegionType::AcpiNvs => warnln!("This is AcpiNvs {} to {}", start, end),
            MemoryRegionType::AcpiReclaimable => warnln!("This is AcpiReclaimable {} to {}", start, end),
            MemoryRegionType::FrameZero => warnln!("This is Frame Zero {} to {}", start, end),
            MemoryRegionType::KernelStack => warnln!("This is the kernel stack {} to {}", start, end),
            _ => warnln!("region undefined")
        }
    }
}