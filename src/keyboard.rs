use core::prelude::*;

use idt;
use io;
use irq;
use vga;

const KEYBORD_INTERRUPT: uint = 33;

const CAPS_LOCK: u8 = 0x3a;
const LSHIFT: u8 = 0x2a;
const RSHIFT: u8 = 0x36;
const NUMBER_LOCK: u8 = 0x45;
const SCROLL_LOCK: u8 = 0x46;


static KEYMAP: &'static str = "\x00\x1B1234567890-=\x08\tqwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?????????????789-456+1230.?????";
static mut KEYBOARD: Keyboard = Keyboard { caps: false, shift: false };


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

    fn write(&self, scancode: u8) {
        if scancode > KEYMAP.len() as u8 {
            return
        }

        let character = match scancode {
            16 => 'q',
            17 => 'w',
            18 => 'e',
            19 => 'r',
            _  => '?'
        };

        let mut vga = vga::VGA::new();
        vga.putc(character as u8);
    }
}


fn handler(_: idt::Registers) {
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

pub fn init() {
    irq::register(1, handler);
    irq::enable(1);
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
