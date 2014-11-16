use core::str::StrPrelude;


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

struct Cursor {
    height: u16,
    width: u16,
    x: u16,
    y: u16,
}

impl Cursor {

    pub fn new() -> Cursor {
        Cursor {height: VGA_HEIGHT, width: VGA_WIDTH, x: 0, y: 0}
    }

    pub fn back(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn forward(&mut self) {
        self.x += 1;

        if self.x >= self.width {
            self.newline();
        }
    }

    pub fn cr(&mut self) {
        self.x = 0;
    }

    pub fn newline(&mut self) {
        self.x = 0;
        self.y += 1;
    }

    pub fn offset(&self) -> u16 {
        self.y * self.width + self.x
    }

    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.mov();
    }

    // handle a tab by increasing the cursor's X, but only to a point
    // where it is divisible by 8
    pub fn tab(&mut self) {
        self.x = (self.x + 8) & !(8 - 1);
    }

    pub fn mov(&mut self) {
        let offset = self.offset() as u8;

        unsafe {
            self.outb(0x3D4, 14);  // tell the VGA board we are setting the high cursor byte
            self.outb(0x3D5, offset >> 8); // send the high cursor byte
            self.outb(0x3D4, 15);  // tell the VGA board we are setting the low cursor byte
            self.outb(0x3D5, offset);
        }
    }

    unsafe fn outb(&mut self, port: u16, value: u8) {
        asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile" );
    }
}

pub struct VGA {
    cursor: Cursor
}

impl VGA {
    pub fn new() -> VGA {
        VGA {cursor: Cursor::new()}
    }

    pub fn clear (&mut self) {
        let mut x: u16 = 0;

        loop {
            if x > 80 * 200 {
                break;
            }

            self.put(x, Black as u16, White as u16, WHITESPACE);
            x += 1;
        }
    }

    pub fn put(&mut self, offset: u16, background: u16, foreground: u16, character: u8) {
        let pixel: u16 = (background << 12) | (foreground << 8) | character as u16;

        unsafe {
                *((VGA_ADDRESS + offset as int * 2) as *mut u16) = pixel;
            }
    }

    pub fn putc(&mut self, character: u8) {

        if character == BACKSPACE {
            self.cursor.back();
        }

        else if character == TAB {
            self.cursor.tab();
        }

        else if character == NEWLINE {
            self.cursor.newline();
        }

        else if character == CR {
            self.cursor.cr();
        }

        else if character >= WHITESPACE {
            let offset = self.cursor.offset();
            self.put(offset, Black as u16, White as u16, character);
            self.cursor.forward();
        }

        self.cursor.mov();
   }

    pub fn puts(&mut self, string: &str) {
        for character in string.bytes() {
            self.putc(character);
        }

    }
}
