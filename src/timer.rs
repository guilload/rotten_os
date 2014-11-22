use idt;
use io;
use irq;


const FREQUENCY: u32 = 100;  // hz
const PIT_FREQUENCY: u32 = 1193180;
const DIVISOR: u32 = PIT_FREQUENCY / FREQUENCY;

const LO: u8 = (DIVISOR & 0xFF) as u8;
const HI: u8 = (DIVISOR >> 8) as u8;


fn handler(_: idt::Registers) {

}

pub fn init() {
    io::port::write(0x43, 0x36);
    io::port::write(0x40, LO);
    io::port::write(0x40, HI);

    irq::register(0, handler);
    irq::enable(0);
}
