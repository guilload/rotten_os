pub mod port {

    pub fn write(port: u16, value: u8) {
        unsafe {
            asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile");
        }
    }
}
