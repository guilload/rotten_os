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
type ISRHandler = fn(registers: Registers);


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
}


pub fn init() {
    IDT::new().load();
}

fn register_handler(no: uint, func: ISRHandler) {
    unsafe {
        IDT_TABLE[no] = IDTDescriptor::new(isr_handlers[no], 0x08, 0x8E);
        ISR_HANDLERS[no] = func;
    }
}

fn default_handler(registers: Registers) {
    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Unhandled interrupt: ");
    vga.puti(registers.interrupt as uint);
    vga.puts(", error: ");
    vga.puti(registers.error as uint);
}

pub fn handle(registers: Registers) {
    let isr = registers.interrupt;

    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Handling interrupt: ");
    vga.puti(registers.interrupt as uint);
    vga.puts(", error: ");
    vga.puti(registers.error as uint);

    if isr > 255 {
        return
    }

    if 31 < isr && isr < 48 {  // IRQ
        let irq = isr - 32;

        if irq < 8 {
            io::port::write(0x20, 0x20);  // master
        }

        io::port::write(0xA0, 0x20);  // slave
    }

    unsafe {
        ISR_HANDLERS[isr as uint](registers);
    }
}


#[repr(C)]
pub struct Registers {
    ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    pub interrupt: u32,
    error: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32,
}
