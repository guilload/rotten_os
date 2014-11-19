use vga;


#[repr(packed)]
struct Registers {
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
    interrupt_number: u32,
    error_code: u32,
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
   vga.puts("interrupt received!");
}
