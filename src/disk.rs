use x86_64::instructions::port::Port;

use crate::{infoln, warnln};

pub fn write_sector(lba: u32, data: &[u16]) {
    assert!(data.len() == 256, "ATA sector size must be 512 bytes (256 words) :3");

    let mut data_port = Port::<u16>::new(0x1F0);
    let mut sector_count_port = Port::<u8>::new(0x1F2);
    let mut lba_low_port = Port::<u8>::new(0x1F3);
    let mut lba_mid_port = Port::<u8>::new(0x1F4);
    let mut lba_high_port = Port::<u8>::new(0x1F5);
    let mut drive_head_port = Port::<u8>::new(0x1F6);
    let mut command_port = Port::<u8>::new(0x1F7);
    let mut status_port = Port::<u8>::new(0x1F7);

    unsafe {
        drive_head_port.write(0xE0 | ((lba >> 24) & 0x0F) as u8);

        sector_count_port.write(1);

        lba_low_port.write((lba & 0xFF) as u8);
        lba_mid_port.write(((lba >> 8) & 0xFF) as u8);
        lba_high_port.write(((lba >> 16) & 0xFF) as u8);

        command_port.write(0x30);

        while status_port.read() & 0x80 != 0 {}

        for &word in data {
            data_port.write(word);
        }

        command_port.write(0xE7);
    }

    for _ in 0..100000 { x86_64::instructions::nop(); }
}

pub fn read_sector(lba: u32, buffer: &mut [u16]) {
    assert!(buffer.len() == 256, "Buffer must hold 512 bytes (256 words):3");

    let mut data_port = Port::<u16>::new(0x1F0);
    let mut sector_count_port = Port::<u8>::new(0x1F2);
    let mut lba_low_port = Port::<u8>::new(0x1F3);
    let mut lba_mid_port = Port::<u8>::new(0x1F4);
    let mut lba_high_port = Port::<u8>::new(0x1F5);
    let mut drive_head_port = Port::<u8>::new(0x1F6);
    let mut command_port = Port::<u8>::new(0x1F7);
    let mut status_port = Port::<u8>::new(0x1F7);

    unsafe {
        drive_head_port.write(0xE0 | ((lba >> 24) & 0x0F) as u8);

        sector_count_port.write(1);

        lba_low_port.write((lba & 0xFF) as u8);
        lba_mid_port.write(((lba >> 8) & 0xFF) as u8);
        lba_high_port.write(((lba >> 16) & 0xFF) as u8);

        command_port.write(0x20);

        while status_port.read() & 0x80 != 0 {}

        for word in buffer.iter_mut() {
            *word = data_port.read();
        }
    }
}

pub fn get_sector_count() -> u32 {
    let mut command_port = Port::<u8>::new(0x1F7);
    let mut data_port = Port::<u16>::new(0x1F0);
    
    unsafe { command_port.write(0xEC) };

    while unsafe { command_port.read() } & 0x80 != 0 {}

    let mut sector_count: u32 = 0;
    for i in 0..100 {
        let word = unsafe { data_port.read() };
        if i == 60 {
            sector_count |= word as u32;
        } else if i == 61 {
            sector_count |= (word as u32) << 16;
        }
    }

    unsafe { command_port.write(0xE7) };
    for _ in 0..100000 { x86_64::instructions::nop(); }

    sector_count
}

#[warn(asm_sub_register)]
fn check_ring() -> i16 {
    let cpl: i16;
    unsafe { core::arch::asm!("mov {0:x}, cs", out(reg) cpl) };
    cpl & 0b11
}

pub fn print_ring() {
    let ring = check_ring();

    match ring {
        0 => infoln!("[YAY] Current ring: 0 (Kernel)"),
        3 => warnln!("[AWW] Current ring: 0 (User)"),
        _ => warnln!("[AWW] Current ring: {} (Likely protected)", ring)
    }
}