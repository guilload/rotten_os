use core::prelude::*;

use idt;
use io;
use vga;

const KEYBORD_INTERRUPT: uint = 33;

const CAPS_LOCK: u8 = 0x3a;
const LSHIFT: u8 = 0x2a;
const RSHIFT: u8 = 0x36;
const NUMBER_LOCK: u8 = 0x45;
const SCROLL_LOCK: u8 = 0x46;


static mut KEYBOARD: Keyboard = Keyboard { caps: false, shift: false };

// use arch::irq;
// use arch::idt;

// use drivers::vga;


struct Keyboard {
    caps: bool,
    shift: bool,
}

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard { caps: false, shift: false }
    }

    fn keyup(&mut self, scancode: u8) {
        match scancode {
            LSHIFT | RSHIFT => self.shift = false,
            _ => {}
        }
    }

    fn keydown(&mut self, scancode: u8) {
        match scancode {
            LSHIFT | RSHIFT => self.shift = true,
            CAPS_LOCK => self.caps = !self.caps,
            _ => self.write(scancode),
        }
    }

    fn register(&self, handler: idt::ISRHandler) {
        idt::register_handler(KEYBORD_INTERRUPT, handler);
    }

    fn write(&self, scancode: u8) {
        // if scancode > KEYMAP.len() as u8 {
        //     return
        // }

        let mut vga = vga::VGA::new();
        vga.clear();
        vga.putc(scancode);
    }
}

pub fn init() {
    unsafe { KEYBOARD.register(handle); }
}

fn handle(_: idt::Registers) {
    let status: u8 = io::port::read(0x64);

    if status & 0x1 == 0 {
        return
    }

    let scancode: u8 = io::port::read(0x60);

    if scancode & 0x80 != 0 { // top bit means key released
        unsafe { KEYBOARD.keyup(!0x80); }
    }

    else {
        unsafe { KEYBOARD.keydown(scancode) };
    }
}


//     let c: char = unsafe {
//         if shifted ^ caps_lock {
//             KEYMAP_SHIFTED.char_at(scancode as uint)
//         } else {
//             KEYMAP.char_at(scancode as uint)
//         }
//     };

//     vga::putch(c);


// }

// static KEYMAP: &'static str = "\
// \x00\x1B1234567890-=\x08\tqwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?????????????789-456+1230.?????";
// static KEYMAP_SHIFTED: &'static str = "\
// \x00\x1B!@#$%^&*()_+\x08\tQWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?????????????789-456+1230.?????";
