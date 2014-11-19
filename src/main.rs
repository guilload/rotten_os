#![crate_type = "lib"]
#![no_std]
#![allow(improper_ctypes)]
#![feature(asm, lang_items, phase, globs)]

#[phase(plugin, link)]

extern crate core;

mod gdt;
mod idt;
mod vga;


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }


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

#[no_mangle]
pub extern "C" fn isr_handler(registers: Registers)
{
   let mut vga = vga::VGA::new();
   vga.clear();
   vga.puts("interrupt received!");
}

#[no_mangle]
#[no_stack_check]
pub extern fn kmain() {
    gdt::init();
    idt::init();
    unsafe {
        asm!("sti");
    }

    let mut vga = vga::VGA::new();
    vga.clear();
    vga.puts("Hello, world!");

    // unsafe {
    //     asm!("int 0x3");
    //     asm!("int 0x4");
    // }
}
