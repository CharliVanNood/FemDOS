use x86_64::instructions::port::Port;
use spin::Mutex;

use crate::println;

fn read_rtc_register(reg: u8) -> u8 {
    unsafe {
        let mut port_70 = Port::<u8>::new(0x70);
        let mut port_71 = Port::<u8>::new(0x71);
        port_70.write(reg);
        port_71.read()
    }
}

fn bcd_to_binary(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

lazy_static::lazy_static! {
    static ref TIME_OFFSET: Mutex<(i8, i8)> = Mutex::new((0, 0));
}

pub fn get_time() -> (u8, u8, u8) {
    let time_offset = TIME_OFFSET.lock().clone();
    let mut current_hour = bcd_to_binary(read_rtc_register(0x04)) as i8 + time_offset.0;

    if current_hour > 23 {
        current_hour = current_hour - 24;
    } else if current_hour < 0 {
        current_hour = 24 + current_hour;
    }

    (current_hour as u8, bcd_to_binary(read_rtc_register(0x02)), bcd_to_binary(read_rtc_register(0x00)))
}

pub fn print_time() {
    let time_offset = TIME_OFFSET.lock().clone();
    let mut current_hour = bcd_to_binary(read_rtc_register(0x04)) as i8 + time_offset.0;

    if current_hour > 23 {
        current_hour = current_hour - 24;
    } else if current_hour < 0 {
        current_hour = 24 + current_hour;
    }

    println!("{:02}:{:02}:{:02}", current_hour, 
                                    bcd_to_binary(read_rtc_register(0x02)), 
                                    bcd_to_binary(read_rtc_register(0x00)));
}

pub fn set_time(hour: u8) {
    let current_hour = bcd_to_binary(read_rtc_register(0x04));
    let offset = hour as i8 - current_hour as i8;
    TIME_OFFSET.lock().0 = offset;
}