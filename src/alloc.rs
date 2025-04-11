use core::ptr;
use bootloader::{bootinfo, BootInfo};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{println, warnln, infoln};
use crate::alloc::bootinfo::MemoryRegionType;

pub struct Allocator {
    heap_start: usize,
    heap_end: usize,
    heap_size: usize,
    next: usize,
}

impl Allocator {
    pub fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_end: heap_start + heap_size,
            heap_size: heap_size,
            next: heap_start,
        }
    }

    fn set_heap(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
        self.heap_size = heap_size;
    }

    pub fn alloc(&mut self, size_raw: usize) -> (usize, usize) {
        let size = size_raw * 8;
        if self.next + size > self.heap_end {
            warnln!("Address 0x{:x} is out of range", self.next + size);
            return (0, 0);
        }

        self.next += size;
        (self.next - size, self.next)
    }

    pub fn shift_back(&mut self, heap_start: usize, new_heap_start: usize) {
        for i in 0..heap_start - new_heap_start {
            unsafe {
                let previous_bit = ptr::read((heap_start + i * 8) as *mut usize);
                ptr::write((new_heap_start + i * 8) as *mut usize, previous_bit);
            }
        }
        self.next -= heap_start - new_heap_start;
    }

    pub fn unalloc(&mut self, heap_start: usize, heap_size: usize) {
        if heap_start + heap_size == self.next {
            self.next -= heap_size;
            for i in 0..heap_size {
                unsafe {
                    ptr::write((heap_start + i * 8) as *mut usize, 0);
                }
            }
        } else {
            self.shift_back(heap_start + heap_size, heap_start);
        }
    }
}

lazy_static! {
    pub static ref ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new(0, 0));
}

pub fn get_usage() -> (usize, usize) {
    let next = { ALLOCATOR.lock().next };
    let heap_start = { ALLOCATOR.lock().heap_start };
    let heap_size = { ALLOCATOR.lock().heap_size };
    (next - heap_start, heap_size)
}

pub fn set_heap(heap_start: usize, heap_size: usize) {
    ALLOCATOR.lock().set_heap(heap_start, heap_size);
}

pub fn alloc(size: usize) -> (usize, usize) {
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