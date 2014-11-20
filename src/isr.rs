#[no_mangle]
pub extern "C" fn isr_handler(&mut registers: Registers)
{
   let mut vga = vga::VGA::new();
   vga.clear();
   vga.puts("interrupt received!");
}

extern {
    fn idt_load(pointer: *const IDT);
}
