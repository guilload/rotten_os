use io;

const IRQ_OFFSET: u8 = 0x20;

const MASTER_CMD: u16 = 0x20;
const MASTER_DATA: u16 = 0x21;
const SLAVE_CMD: u16 = 0xA0;
const SLAVE_DATA: u16 = 0xA1;

const IRQ_ACK: u8 = 0x20;
const ICW1: u8 = 0x11;
const ICW4: u8 = 0x01;


pub fn init() {
    unsafe {
        // Initialize
        io::port::write(MASTER_CMD, ICW1);
        io::port::wait();
        io::port::write(SLAVE_CMD, ICW1);
        io::port::wait();

        // Set offset
        io::port::write(MASTER_DATA, IRQ_OFFSET);
        io::port::wait();
        io::port::write(SLAVE_DATA, IRQ_OFFSET + 8);
        io::port::wait();

        // Connect Master to slave
        io::port::write(MASTER_DATA, 4);
        io::port::wait();
        io::port::write(SLAVE_DATA, 2);
        io::port::wait();

        // Finalize
        io::port::write(MASTER_DATA, ICW4);
        io::port::wait();
        io::port::write(SLAVE_DATA, ICW4);
        io::port::wait();

        // Disable all interrupts
        io::port::write(MASTER_DATA, 0xFF);
        io::port::write(SLAVE_DATA, 0xFF);
    }
}
