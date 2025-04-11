#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fem_dos::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod window;
mod input;
mod applications;
mod renderer;
mod vec;
mod filesystem;
mod disk;
mod string;
mod alloc;
mod clock;

use core::panic::PanicInfo;
use bootloader::BootInfo;

use alloc::{read_byte, write_byte};
use fem_dos::alloc::alloc;
use vec::Vec;

const VERSION: &str = env!("VERSION");

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    window::init();
    window::clear_screen();

    println!("-------------------------");
    println!("FemDOS!");
    println!("Data:");
    println!("Version: {}", VERSION);
    println!("Mem Offset: 0x{:x}", boot_info.physical_memory_offset);
    println!("-------------------------");

    alloc::set_heap(boot_info.physical_memory_offset as usize + 0x8a5000, 0x7fe0000 - 0x8a5000);
    fem_dos::init(boot_info);

    println!("Initialized components!");

    #[cfg(test)]
    test_main();

    let address = alloc::alloc(1);
    write_byte(address.0, 255);
    let test_byte = read_byte(address.0);
    if test_byte == 255 {
        infoln!("[YAY] Ram test");
    } else {
        warnln!("[AWW] Ram test");
    }

    disk::print_ring();

    let sectors = disk::get_sector_count();
    println!("Amount of sectors: {}", sectors);
    println!("Disk size: {} MB", sectors as u64 * 512 / 1024 / 1024);

    let mut read_buffer = [0u16; 256];
    disk::read_sector(0, &mut read_buffer);

    let write_buffer = [0xABCDu16; 256];
    disk::write_sector(1, &write_buffer);
    disk::read_sector(1, &mut read_buffer);
    let write_successfull = read_buffer == [0xABCDu16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 1");
    } else {
        warnln!("[AWW] Disk write sector 1");
    }

    let write_buffer = [0x1234u16; 256];
    disk::write_sector(2, &write_buffer);
    disk::read_sector(2, &mut read_buffer);
    let write_successfull = read_buffer == [0x1234u16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 2");
    } else {
        warnln!("[AWW] Disk write sector 2");
    }

    let write_buffer = [0x5678u16; 256];
    disk::write_sector(3, &write_buffer);
    disk::read_sector(3, &mut read_buffer);
    let write_successfull = read_buffer == [0x5678u16; 256];
    if write_successfull {
        infoln!("[YAY] Disk write sector 3");
    } else {
        warnln!("[AWW] Disk write sector 3");
    }

    let write_buffer = [0x1369u16; 256];
    disk::write_sector(4, &write_buffer);
    disk::read_sector(4, &mut read_buffer);
    let write_successfull = read_buffer == [0xABCDu16; 256];
    if !write_successfull {
        infoln!("[YAY] Disk write sector 4");
    } else {
        warnln!("[AWW] Disk write sector 4");
    }

    let mut test_vec_1 = Vec::new();
    test_vec_1.add(1);
    test_vec_1.add(2);
    test_vec_1.add(3);
    test_vec_1.remove();

    let mut test_vec_2 = Vec::new();
    test_vec_2.add(4);
    test_vec_2.add(5);
    test_vec_2.add(6);
    test_vec_2.remove();

    let ram_usage = alloc::get_usage();
    if ram_usage.0 == 8 {
        infoln!("[YAY] Heap vectors");
    } else {
        warnln!("[AWW] Heap vectors");
    }

    /*for region in boot_info.memory_map.iter() {
        println!(
            "Address is mapped as {:?} at {:x} to {:x}",
            region.region_type, region.range.start_addr(), region.range.end_addr()
        );
    }*/

    println!("Done testing!");

    println!("-------------------------");
    println!("| Yippee FemDOS booted! |");
    println!("-------------------------");

    fem_dos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    warnln!("{}", info);
    fem_dos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fem_dos::test_panic_handler(info)
}