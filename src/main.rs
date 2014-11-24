#![crate_type = "lib"]
#![no_std]
#![allow(improper_ctypes)]
#![feature(asm, lang_items, phase, globs)]

#[phase(plugin, link)]

extern crate core;

mod cpu;
mod gdt;
mod idt;
mod io;
mod irq;
mod keyboard;
mod pic;
mod timer;
mod vga;


#[no_mangle]
pub extern fn kmain() {
    gdt::init();
    pic::init();
    idt::init();

    keyboard::init();
    timer::init();

    vga::clear();
    vga::puts("Hello, world!\n");

    cpu::idle();
}


#[no_mangle]
pub extern fn handle_interrupt(registers: idt::Registers) {
    idt::handle(registers);
}

#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind() -> ! {
    loop {
    }
}
