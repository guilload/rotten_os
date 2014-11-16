#![crate_type = "lib"]
#![no_std]
#![allow(improper_ctypes)]
#![feature(asm, lang_items, phase, globs)]

#[phase(plugin, link)]

extern crate core;

mod vga;


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[no_mangle]
#[no_stack_check]
pub extern fn kmain() {
    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Hello, world!");

    loop {

    }
}
