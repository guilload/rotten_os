use core::prelude::*;

use idt;
use io;
use irq;
use vga;


const CAPS_LOCK: u8 = 0x3a;
const LSHIFT: u8 = 0x2a;
const RSHIFT: u8 = 0x36;
const NUMBER_LOCK: u8 = 0x45;
const SCROLL_LOCK: u8 = 0x46;


const KEYMAP: &'static str = "\x00\x1B1234567890-=\x08\tqwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?????????????789-456+1230.?????";
const KEYMAP_SHIFTED: &'static str = "\x00\x1B!@#$%^&*()_+\x08\tQWERTYUIOP{}\n?ASDFGHJKL:\"~?|ZXCVBNM<>??*? ?????????????789-456+1230.?????";


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
        if scancode as uint > KEYMAP.len() {
            return
        }

        let keymap = if self.shift ^ self.caps { KEYMAP_SHIFTED } else { KEYMAP };
        let character = unsafe { keymap.char_at(scancode as uint) };

        vga::putc(character as u8);
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
