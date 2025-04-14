use pc_keyboard::KeyCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::hlt_loop;
use crate::infoln;
use crate::warnln;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use spin::Mutex;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::instructions::port::Port;

//use crate::print;
use crate::gdt;
use crate::input;
//use crate::disk;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = 
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[46].set_handler_fn(ata_irq_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    warnln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn ata_irq_handler(_stack_frame: InterruptStackFrame) {
    let mut status_port = Port::<u8>::new(0x1F7);
    let status = unsafe { status_port.read() };

    if status & 0x01 != 0 {
        let mut error_port = Port::<u8>::new(0x1F1);
        let error_code = unsafe { error_port.read() };
        warnln!("[AWW] ATA Error! Error Code: {:#X}", error_code);
    } else {
        infoln!("[YAY] ATA Write Completed Successfully!");
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    warnln!("EXCEPTION: PAGE FAULT");
    warnln!("Accessed Address: {:?}", Cr2::read());
    warnln!("Error Code: {:?}", error_code);
    warnln!("{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: InterruptStackFrame)
{
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(),
                layouts::Us104Key, HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    let byte = character as u8;
                    {
                        let mut keypresses = input::KEYPRESSES.lock();
                        let keypress_index = keypresses.1;
                        keypresses.0[keypress_index as usize] = byte as u16;
                        keypresses.1 += 1;
                        if keypresses.1 > 7 {
                            keypresses.1 = 7;
                        }
                    }
                },
                DecodedKey::RawKey(character) => {
                    match character {
                        KeyCode::ArrowUp => {
                            let mut keypresses = input::KEYPRESSES.lock();
                            let keypress_index = keypresses.1;
                            keypresses.0[keypress_index as usize] = 256;
                            keypresses.1 += 1;
                            if keypresses.1 > 7 {
                                keypresses.1 = 7;
                            }
                        },
                        KeyCode::ArrowDown => {
                            let mut keypresses = input::KEYPRESSES.lock();
                            let keypress_index = keypresses.1;
                            keypresses.0[keypress_index as usize] = 257;
                            keypresses.1 += 1;
                            if keypresses.1 > 7 {
                                keypresses.1 = 7;
                            }
                        },
                        KeyCode::ArrowLeft => {
                            let mut keypresses = input::KEYPRESSES.lock();
                            let keypress_index = keypresses.1;
                            keypresses.0[keypress_index as usize] = 258;
                            keypresses.1 += 1;
                            if keypresses.1 > 7 {
                                keypresses.1 = 7;
                            }
                        },
                        KeyCode::ArrowRight => {
                            let mut keypresses = input::KEYPRESSES.lock();
                            let keypress_index = keypresses.1;
                            keypresses.0[keypress_index as usize] = 259;
                            keypresses.1 += 1;
                            if keypresses.1 > 7 {
                                keypresses.1 = 7;
                            }
                        },
                        _ => {}
                    }
                },
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}