use core::prelude::*;

use io;


const BACKSPACE: u8 = 0x08;
const TAB: u8 = 0x09;
const NEWLINE: u8 = 0x0A;
const CR: u8 = 0x0D;
const WHITESPACE: u8 = 0x20;

const VGA_ADDRESS: int = 0xB8000;
const VGA_HEIGHT: u16 = 25;
const VGA_WIDTH: u16 = 80;


enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Pink = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightPink = 13,
    Yellow = 14,
    White = 15,
}


static mut VGA: VGA = VGA { height: VGA_HEIGHT, width: VGA_WIDTH, x: 0, y: 0 };

struct VGA {
    height: u16,
    width: u16,
    x: u16,
    y: u16,
}

impl VGA {

    fn new() -> VGA {
        VGA {height: VGA_HEIGHT, width: VGA_WIDTH, x: 0, y: 0}
    }

    fn back(&mut self) {

        if self.x > 0 {
            self.x -= 1;

            let offset = self.offset();
            self.put(offset, Black as u16, White as u16, WHITESPACE);
        }
    }

    fn forward(&mut self) {
        self.x += 1;

        if self.x >= self.width {
            self.newline();
        }
    }

    fn cr(&mut self) {
        self.x = 0;
    }

    fn newline(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    fn offset(&self) -> u16 {
        self.y * self.width + self.x
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.mov();
    }

    // handle a tab by increasing the cursor's X, but only to a point
    // where it is divisible by 8
    fn tab(&mut self) {
        self.x = (self.x + 8) & !(8 - 1);
    }

    fn mov(&mut self) {
        let offset = self.offset() as u8;

        io::port::write(0x3D4, 14);  // tell the VGA board we are setting the high cursor byte
        io::port::write(0x3D5, offset >> 8); // send the high cursor byte
        io::port::write(0x3D4, 15);  // tell the VGA board we are setting the low cursor byte
        io::port::write(0x3D5, offset);
    }

    fn clear (&mut self) {
        let mut x: u16 = 0;

        loop {
            if x > 80 * 200 {
                break;
            }

            self.put(x, Black as u16, White as u16, WHITESPACE);
            x += 1;
        }
    }

    fn put(&mut self, offset: u16, background: u16, foreground: u16, character: u8) {
        let pixel: u16 = (background << 12) | (foreground << 8) | character as u16;

        unsafe {
                *((VGA_ADDRESS + offset as int * 2) as *mut u16) = pixel;
            }
    }

    fn puti(&mut self, integer: uint) {  // FIXME
        let mut integer = integer;

        loop {
            self.putc((integer % 10) as u8 + '0' as u8);
            integer = integer / 10;

            if integer == 0 {
                break;
            }
        }
    }

    fn putc(&mut self, character: u8) {

        if character == BACKSPACE {
            self.back();
        }

        else if character == TAB {
            self.tab();
        }

        else if character == NEWLINE {
            self.newline();
        }

        else if character == CR {
            self.cr();
        }

        else if character >= WHITESPACE {
            let offset = self.offset();
            self.put(offset, Black as u16, White as u16, character);
            self.forward();
        }

        self.mov();
   }

    fn puth(&mut self, integer: uint) {
        self.puts("0x");

        let mut nibbles = 1u;

        while (integer >> nibbles * 4) > 0 {
                nibbles += 1
        }

        for i in range(0, nibbles) {
            let nibble = ((integer >> (nibbles - i - 1) * 4) & 0xF) as u8;
            let character = if nibble < 10 { '0' as u8 + nibble } else { 'a' as u8 + nibble - 10 };
            self.putc(character);
        }
    }

    fn puts(&mut self, string: &str) {
        for character in string.bytes() {
            self.putc(character);
        }

    }
}


pub fn clear() {
    unsafe {
        VGA.clear();
    }
}

pub fn puth(integer: uint) {
    unsafe {
        VGA.puth(integer);
    }
}

pub fn puti(integer: uint) {
    unsafe {
        VGA.puti(integer);
    }
}

pub fn putc(character: u8) {
    unsafe {
        VGA.putc(character);
    }
}

pub fn puts(string: &str) {
    unsafe {
        VGA.puts(string);
    }
}
