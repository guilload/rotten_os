#![crate_type = "lib"]
#![no_std]
#![allow(improper_ctypes)]
#![feature(asm, lang_items, phase, globs)]

#[phase(plugin, link)]

extern crate core;

mod gdt;
mod idt;
mod io;
mod vga;


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern fn kmain() {
    gdt::init();
    idt::init();

    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Hello, world!");

    unsafe {
        asm!("int $$0x03");
    }
}


#[no_mangle]
pub extern fn isr_handler(registers: idt::Registers)
{
   idt::handle(registers);
}
