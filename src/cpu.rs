pub fn halt() -> ! {
    loop {
        unsafe {
            asm!("cli");
            asm!("hlt");
        }
    }
}


pub fn idle() {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
