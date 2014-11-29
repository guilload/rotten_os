use core::intrinsics::transmute;
use core::prelude::*;

use idt;
use vga;


const ABSENT: uint = 0;
const PRESENT: uint = 1 << 0;

const READ: uint = 0;
const WRITE: uint = 1 << 1;

const KERNEL: uint = 0;
const USER: uint = 1 << 2;

type Table = [u32, ..1024];


extern {
    fn paging_enable();
    fn paging_load();

    static mut pdirectory: Table;
    static mut ptable: Table;
}


pub fn init() {
    unsafe {

        for i in range(0, pdirectory.len()) {
            pdirectory[i] = (KERNEL | WRITE | ABSENT) as u32;
        }

        for i in range(0, ptable.len()) {
            ptable[i] = (i * 0x1000 | KERNEL | WRITE | PRESENT) as u32;
        }

        // ptable[1] = (1 * 0x1000 | KERNEL | WRITE | ABSENT) as u32;

        pdirectory[0] = transmute(&ptable);
        pdirectory[0] |= (KERNEL | WRITE | PRESENT) as u32;

        idt::register(14, handler);

        paging_load();
        paging_enable();
    }
}

fn handler(_: idt::Registers) {
    let address: u32;

    unsafe {
        asm!("mov %cr2, $0" : "=r" (address));
    }

    vga::puts("Page fault!\n");  // FIXME panic!
    loop {

    }
}
