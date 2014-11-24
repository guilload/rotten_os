#![crate_type = "lib"]
#![no_std]
#![allow(improper_ctypes)]
#![feature(asm, lang_items, phase, globs)]

#[phase(plugin, link)]

extern crate core;

mod gdt;
mod idt;
mod io;
mod irq;
mod keyboard;
mod pic;
mod timer;
mod vga;


fn idle() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}


#[no_mangle]
pub extern fn kmain() {
    gdt::init();
    pic::init();
    idt::init();

    keyboard::init();
    timer::init();

    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Hello, world!");

    idle();
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
