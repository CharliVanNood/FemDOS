//use lazy_static::lazy_static;
//use spin::Mutex;

//use core::arch::asm;
use crate::{infoln, print, println, warnln};
use x86_64::instructions::port::Port;

/*
pub unsafe fn outb(port: u16, value: u8) {
    asm!("out dx, al", in("dx") port, in("al") value, options(nostack, nomem));
}

pub unsafe fn inb(port: u16) -> u8 {
    let result: u8;
    asm!("in al, dx", out("al") result, in("dx") port, options(nostack, nomem));
    result
}

pub unsafe fn outw(port: u16, value: u16) {
    asm!("out dx, ax", in("dx") port, in("ax") value, options(nostack, nomem));
}

pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    asm!("in ax, dx", out("ax") result, in("dx") port, options(nostack, nomem));
    result
}

pub unsafe fn _ata_write_sector(lba: u32, buffer: &[u16]) {
    while (inb(0x1F7) & 0x80) != 0 {}

    outb(0x1F6, ((lba >> 24) as u8) | 0xE0);

    outb(0x1F2, 1);
    outb(0x1F3, (lba & 0xFF) as u8);
    outb(0x1F4, ((lba >> 8) & 0xFF) as u8);
    outb(0x1F5, ((lba >> 16) & 0xFF) as u8);

    outb(0x1F7, 0x30);

    while (inb(0x1F7) & 0x08) == 0 {}

    for &word in buffer.iter().take(256) {
        outw(0x1F0, word);
    }

    outb(0x1F7, 0xE7);

    while (inb(0x1F7) & 0x80) != 0 {print!(".")}
}

pub unsafe fn _ata_read_sector(lba: u32, buffer: &mut [u16]) {
    println!("Waiting for Disk");
    while (inb(0x1F7) & 0x80) == 0 {
        let status = inb(0x1F7);
        if (status & 0x01) != 0 {
            warnln!("Error detected while loading disk!");
            break;
        }
    }
    let status = inb(0x1F7);
    println!("Status after loading: 0x{:X}", status);
    println!("Disk is ready!");

    outb(0x1F6, ((lba >> 24) as u8) | 0xE0);
    println!("Selected drive port 0x1F6");

    outb(0x1F2, 1);
    outb(0x1F3, (lba & 0xFF) as u8);
    outb(0x1F4, ((lba >> 8) & 0xFF) as u8);
    outb(0x1F5, ((lba >> 16) & 0xFF) as u8);
    println!("Set LBA address");
    
    outb(0x1F7, 0x04);
    println!("Resetting drive");
    while (inb(0x1F7) & 0x80) == 0 {}
    while (inb(0x1F7) & 0x08) == 0 {}
    println!("Disk has been reset!");

    outb(0x1F7, 0x20);
    while (inb(0x1F7) & 0x40) != 0 {}

    let status = inb(0x1F7);
    println!("Status read request: 0x{:X}", status);
    println!("Sent read command!");

    println!("Waiting for Data transfer");
    while (inb(0x1F7) & 0x88) != 0x08 {}
    println!("Disk is ready for Data transfer!");

    for word in buffer.iter_mut().take(256) {
        *word = inw(0x1F0);
    }
}


pub fn _check_mbr() -> bool {
    println!("Checking for any mbr data!");
    let mut buffer = [0u16; 256];
    unsafe { ata_read_sector(0, &mut buffer) };
    println!("The first sector has been read :D");

    let signature = (buffer[255] >> 8, buffer[255] & 0xFF);

    if signature == (0x55, 0xAA) {
        println!("There is a signature present!");
    } else {
        println!("No signature has been found");
    }

    signature == (0x55, 0xAA)
}
*/

pub fn _test() {
    let mut port = Port::new(0x1F0);
    unsafe {
        Port::new(0x3F6).write(0x04 as u8);
        Port::new(0x3F6).write(0x00 as u8);
    }

    print!("Identifying drive... ");
    _identify_device();
    println!("Waiting for drive... ");
    _wait_for_ready();
    _write_test_data(&mut port);
    //read_test_data(&mut port);
}

fn _identify_device() {
    unsafe {
        Port::new(0x1F7).write(0xEC as u8);
    }

    loop {
        unsafe {
            let status: u8 = Port::new(0x1F7).read();
            print!("{:#x} ", status);
            if status & 0x80 == 0 {
                if status & 0x01 != 0 {
                    let error: u8 = Port::new(0x1F1).read();
                    warnln!("\nError while identifying device: {:#x}", error);
                    break;
                } else {
                    infoln!("\nStorage device identified successfully.");
                    break;
                }
            }
        }
    }
}

fn _wait_for_ready() {
    let mut status: u8;
    loop {
        unsafe { status = Port::new(0x1F7).read(); }
        println!("{:#x} ", status);
        if status & 0x80 == 0 {
            if status & 0x08 != 0 {
                println!("Drive is ready!");
                break;
            }
        }
        for _ in 0..10000 { unsafe { let _: u16 = Port::new(0x80).read(); } }
    }
}

fn _write_test_data(port: &mut Port<u16>) {
    println!("Writing test data to disk...");
    
    unsafe {
        Port::new(0x1F6).write(0xE0 as u8);
        Port::new(0x1F2).write(1 as u8); // amount of sectors to read
        Port::new(0x1F3).write(10 as u8);
        Port::new(0x1F4).write(0 as u8);
        Port::new(0x1F5).write(0 as u8);
    }

    _wait_for_ready();
    
    unsafe {
        Port::new(0x1F7).write(0x30 as u8);
    }

    
    _wait_for_ready();

    unsafe {
        let status: u8 = Port::new(0x1F7).read();
        if status & 0x01 != 0 {
            println!("Write error detected: Status={:#x}", status);
            return;
        }
    }

    for i in 0..256 {
        unsafe {
            port.write(i as u16);
        }
    }

    /*unsafe {
        Port::new(0x1F7).write(0xE7 as u8);
    }*/

    let mut debug_buffer = [0u16; 128];
    for i in 0..128 {
        unsafe { debug_buffer[i] = port.read(); }
    }

    println!("DEBUG: Readback after write:");
    for (i, word) in debug_buffer.iter().enumerate() {
        print!("{:04x} ", word);
        if (i + 1) % 16 == 0 { println!(); }
    }
}

fn _read_test_data(port: &mut Port<u16>) {
    println!("Reading test data from disk...");

    unsafe {
        Port::new(0x1F6).write(0xE0 as u8);
        Port::new(0x1F2).write(1 as u8); // amount of sectors to read
        Port::new(0x1F3).write(0 as u8);
        Port::new(0x1F4).write(0 as u8);
        Port::new(0x1F5).write(0 as u8);
    }
    
    _wait_for_ready();

    unsafe {
        Port::new(0x1F7).write(0x20 as u8);
    }

    _wait_for_ready();

    let mut buffer = [0u16; 128];
    for i in 0..128 {
        unsafe {
            buffer[i] = port.read();
        }
    }

    println!("Read complete. Data:");
    for (i, word) in buffer.iter().enumerate() {
        print!("{:04x} ", word);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
}