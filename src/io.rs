pub mod port {

    pub fn read(port: u16) -> u8 {
        unsafe {
            let byte: u8;
            asm!("inb %dx, %al" : "={al}" (byte) : "{dx}" (port) :: "volatile");
            byte
        }
    }

    pub fn write(port: u16, value: u8) {
        unsafe {
            asm!("outb %al, %dx" :: "{dx}" (port), "{al}" (value) :: "volatile");
        }
    }

    pub fn wait() {
        write(0x80, 0);
    }
}
