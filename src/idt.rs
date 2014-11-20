use core::mem::size_of;
use core::prelude::*;

use io;
use vga;


extern {
    fn idt_load(pointer: *const IDT);
    static isr_handlers: [u32, ..IDT_SIZE];
}


const IDT_SIZE: uint = 256;

const IDT_ALWAYS14: u8 = 14;
const IDT_PRESENT: u8 = 1 << 7;
const IDT_: u8 = IDT_PRESENT | IDT_ALWAYS14;

const INTERRUPT_GATE: u8 = 0xE;

type IDTTable = [IDTDescriptor, ..IDT_SIZE];
type ISRHandler = fn(registers: &mut Registers)


static mut ISR_HANDLERS: [ISRHandler, ..IDT_SIZE] = [default_handler, ..IDT_SIZE];

static mut IDT_TABLE: IDTTable = [
    IDTDescriptor {
        base_lo: 0,
        sel: 0,
        always0: 0,
        flags: 0,
        base_hi: 0,
    }, ..IDT_SIZE
];


#[repr(packed)]
struct IDTDescriptor {
    base_lo: u16,  // the lower 16 bits of the address to jump to when this interrupt fires
    sel: u16,  // kernel segment selector
    always0: u8,  // this must always be zero
    flags: u8,  // more flags, see documentation
    base_hi: u16,  // the upper 16 bits of the address to jump to
}

impl IDTDescriptor {

    pub fn new(base: u32, sel: u16, flags: u8) -> IDTDescriptor {
        IDTDescriptor {
            base_lo: (base & 0xFFFF) as u16,
            base_hi: (base >> 16 & 0xFFFF) as u16,
            sel: sel,
            always0: 0u8,
            flags: flags,
        }
    }
}


#[repr(packed)]
struct IDT {
    limit: u16,
    base: *const IDTTable,
}

impl IDT {

    pub fn new() -> IDT {

        unsafe {

            for i in range(0, isr_handlers.len()) {
                IDT_TABLE[i] = IDTDescriptor::new(isr_handlers[i], 0x08, 0x8E);
            }

            IDT {
                limit: (size_of::<IDTDescriptor>() * IDT_TABLE.len() - 1) as u16,
                base: &IDT_TABLE as *const IDTTable,
            }
        }
    }

    pub fn load(&self) {
        unsafe {
            idt_load(self as *const IDT);
        }
    }

    pub fn remap_irq(&mut self) {  // Remap the irq table.
        unsafe {
            io::port::write(0x20, 0x11);
            io::port::write(0xA0, 0x11);
            io::port::write(0x21, 0x20);
            io::port::write(0xA1, 0x28);
            io::port::write(0x21, 0x04);
            io::port::write(0xA1, 0x02);
            io::port::write(0x21, 0x01);
            io::port::write(0xA1, 0x01);
            io::port::write(0x21, 0x0);
            io::port::write(0xA1, 0x0);
        }
    }
}


pub fn init() {
    let mut idt = IDT::new();
    idt.load();
    idt.remap_irq();
}

fn register_handler(no: uint, func: fn(regs: &mut Registers)) {
    unsafe {
        IDT_TABLE[no] = IDTDescriptor::new(isr_handlers[no], //FIXME);
        ISR_HANDLERS[no] = func;
    }
}


fn default_handler(registers: &mut Registers) {
    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Unhandled interrupt!");
}


pub fn register_interrupt(number: uint, func: fn(regs: &mut Registers)) {
    register_handler(number, func);
}

#[repr(C)]
pub struct Registers {
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    gs: u32,
    fs: u32,
    es: u32,
    ds: u32,
    interrupt: u32,
    error: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32
}
