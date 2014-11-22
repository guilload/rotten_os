use idt;
use io;
use pic;


const EOI: u8 = 0x20;  // end of interrupt

enum IRQ {
    IRQ0 = 32,
    IRQ1 = 33,
    IRQ2 = 34,
    IRQ3 = 35,
    IRQ4 = 36,
    IRQ5 = 37,
    IRQ6 = 38,
    IRQ7 = 39,
    IRQ8 = 40,
    IRQ9 = 41,
    IRQ10 = 42,
    IRQ11 = 43,
    IRQ12 = 44,
    IRQ13 = 45,
    IRQ14 = 46,
    IRQ15 = 47,
}


pub fn eoi(irq: uint) {
    if irq >= 40 {
        io::port::write(0xA0, 0x20);  // slave
    }

    io::port::write(0x20, 0x20);  // master
}


pub fn enable(irq: uint)
{
    let (port, line) = if irq >= 8 as uint {
        (pic::SLAVE_DATA, irq - 8)
    }

    else {
        (pic::MASTER_DATA, irq)
    };

    let value = io::port::read(port) & !(1 << line);
    io::port::write(port, value);
}


pub fn register(irq: uint, handler: idt::InterruptHandler) {
    idt::register(irq + 32, handler);
}
