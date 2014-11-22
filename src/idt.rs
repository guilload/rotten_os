use core::mem::size_of;
use core::prelude::*;


use irq;
use vga;


const IDT_SIZE: uint = 256;

extern {
    fn idt_load(pointer: *const IDT);
    static interrupt_handlers: [u32, ..IDT_SIZE];
}


#[repr(packed)]
struct IDTDescriptor {
    base_lo: u16,  // the lower 16 bits of the address to jump to when this interrupt fires
    selector: u16,  // kernel segment selector
    zero: u8,  // this must always be zero
    flags: u8,  // more flags, see documentation
    base_hi: u16,  // the upper 16 bits of the address to jump to
}

impl IDTDescriptor {

    pub fn new(base: u32, selector: u16, flags: u8) -> IDTDescriptor {
        IDTDescriptor {
            base_lo: (base & 0xFFFF) as u16,
            selector: selector,
            zero: 0,
            flags: flags,
            base_hi: (base >> 16 & 0xFFFF) as u16,
        }
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

pub type InterruptHandler = fn(registers: Registers);

fn dummy_handler(registers: Registers) {
    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Unhandled interrupt: ");
    vga.puti(registers.interrupt as uint);
    vga.puts(", error: ");
    vga.puti(registers.error as uint);
}

static mut INTERRUPT_HANDLERS: [InterruptHandler, ..IDT_SIZE] = [dummy_handler, ..IDT_SIZE];


type IDTable = [IDTDescriptor, ..IDT_SIZE];

static mut IDTABLE: IDTable = [
    IDTDescriptor {
        base_lo: 0,
        selector: 0,
        zero: 0,
        flags: 0,
        base_hi: 0,
    }, ..IDT_SIZE
];


#[repr(packed)]
struct IDT {
    limit: u16,
    base: *const IDTable,
}

impl IDT {

    fn new() -> IDT {
        unsafe {

            for i in range(0, interrupt_handlers.len()) {
                IDTABLE[i] = IDTDescriptor::new(interrupt_handlers[i] as u32, 0x08, 0x8E);
            }

            IDT {
                limit: (size_of::<IDTDescriptor>() * IDTABLE.len() - 1) as u16,
                base: &IDTABLE as *const IDTable,
            }
        }
    }

    fn load(&self) {
        unsafe {
            idt_load(self as *const IDT);
        }
    }
}


pub fn init() {
    let idt = IDT::new();
    idt.load();
    enable();
}

fn enable() {
    unsafe {
        asm!("sti");
    }
}

pub fn handle(registers: Registers) {
    let interrupt: uint = registers.interrupt as uint;

    if interrupt > 255 {
        return
    }

    if interrupt >= 32 && interrupt <= 47 {
        irq::eoi(interrupt - 32);
    }

    unsafe {
        INTERRUPT_HANDLERS[interrupt](registers);
    }
}

pub fn register(number: uint, handler: InterruptHandler) {
    unsafe {
        IDTABLE[number] = IDTDescriptor::new(interrupt_handlers[number], 0x08, 0x8E);
        INTERRUPT_HANDLERS[number] = handler;
    }
}
