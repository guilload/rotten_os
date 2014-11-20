pub mod port {

    pub unsafe fn write(port: u16, value: u8) {
        asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile" );
    }
}
