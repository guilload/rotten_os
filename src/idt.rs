use core::mem::size_of;
use core::prelude::*;


const IDT_SIZE: uint = 256;

type IDTTable = [IDTEntry, ..IDT_SIZE];


#[repr(packed)]
struct IDTEntry {
    base_lo: u16,  // the lower 16 bits of the address to jump to when this interrupt fires
    sel: u16,  // kernel segment selector
    always0: u8,  // this must always be zero
    flags: u8,  // more flags, see documentation
    base_hi: u16,  // the upper 16 bits of the address to jump to
}

impl IDTEntry {

    pub fn new(base: u32, sel: u16, flags: u8) -> IDTEntry {
        IDTEntry {
            base_lo: (base & 0xFFFF) as u16,
            base_hi: (base >> 16 & 0xFFFF) as u16,
            sel: sel,
            always0: 0u8,
            flags: flags,
        }
    }

    pub fn privileged(base: u32) -> IDTEntry {
        IDTEntry::new(base, 0x08, 0x8E)
    }
}

#[repr(packed)]
struct IDT {
    limit: u16,
    base: *const IDTTable,
}

impl IDT {

    pub fn new() -> IDT {

        unsafe {
            TABLE[0] = IDTEntry::privileged(isr0 as u32);
            TABLE[1] = IDTEntry::privileged(isr1 as u32);
            TABLE[2] = IDTEntry::privileged(isr2 as u32);
            TABLE[3] = IDTEntry::privileged(isr3 as u32);
            TABLE[4] = IDTEntry::privileged(isr4 as u32);
            TABLE[5] = IDTEntry::privileged(isr5 as u32);
            TABLE[6] = IDTEntry::privileged(isr6 as u32);
            TABLE[7] = IDTEntry::privileged(isr7 as u32);
            TABLE[8] = IDTEntry::privileged(isr8 as u32);
            TABLE[9] = IDTEntry::privileged(isr9 as u32);
            TABLE[10] = IDTEntry::privileged(isr10 as u32);
            TABLE[11] = IDTEntry::privileged(isr11 as u32);
            TABLE[12] = IDTEntry::privileged(isr12 as u32);
            TABLE[13] = IDTEntry::privileged(isr13 as u32);
            TABLE[14] = IDTEntry::privileged(isr14 as u32);
            TABLE[15] = IDTEntry::privileged(isr15 as u32);
            TABLE[16] = IDTEntry::privileged(isr16 as u32);
            TABLE[17] = IDTEntry::privileged(isr17 as u32);
            TABLE[18] = IDTEntry::privileged(isr18 as u32);
            TABLE[19] = IDTEntry::privileged(isr19 as u32);
            TABLE[20] = IDTEntry::privileged(isr20 as u32);
            TABLE[21] = IDTEntry::privileged(isr21 as u32);
            TABLE[22] = IDTEntry::privileged(isr22 as u32);
            TABLE[23] = IDTEntry::privileged(isr23 as u32);
            TABLE[24] = IDTEntry::privileged(isr24 as u32);
            TABLE[25] = IDTEntry::privileged(isr25 as u32);
            TABLE[26] = IDTEntry::privileged(isr26 as u32);
            TABLE[27] = IDTEntry::privileged(isr27 as u32);
            TABLE[28] = IDTEntry::privileged(isr28 as u32);
            TABLE[29] = IDTEntry::privileged(isr29 as u32);
            TABLE[30] = IDTEntry::privileged(isr30 as u32);
            TABLE[31] = IDTEntry::privileged(isr31 as u32);

            TABLE[32] = IDTEntry::privileged(irq0 as u32);
            TABLE[33] = IDTEntry::privileged(irq1 as u32);
            TABLE[34] = IDTEntry::privileged(irq2 as u32);
            TABLE[35] = IDTEntry::privileged(irq3 as u32);
            TABLE[36] = IDTEntry::privileged(irq4 as u32);
            TABLE[37] = IDTEntry::privileged(irq5 as u32);
            TABLE[38] = IDTEntry::privileged(irq6 as u32);
            TABLE[39] = IDTEntry::privileged(irq7 as u32);
            TABLE[40] = IDTEntry::privileged(irq8 as u32);
            TABLE[41] = IDTEntry::privileged(irq9 as u32);
            TABLE[42] = IDTEntry::privileged(irq10 as u32);
            TABLE[43] = IDTEntry::privileged(irq11 as u32);
            TABLE[44] = IDTEntry::privileged(irq12 as u32);
            TABLE[45] = IDTEntry::privileged(irq13 as u32);
            TABLE[46] = IDTEntry::privileged(irq14 as u32);
            TABLE[47] = IDTEntry::privileged(irq15 as u32);

            IDT {
                limit: (size_of::<IDTEntry>() * TABLE.len() - 1) as u16,
                base: &TABLE as *const IDTTable,
            }
        }
    }

    pub fn load(&self) {
        unsafe {
            idtload(self as *const IDT);
        }
    }
}


static mut TABLE: IDTTable = [
    IDTEntry {
        base_lo: 0,
        sel: 0,
        always0: 0,
        flags: 0,
        base_hi: 0,
    }, ..IDT_SIZE
];

pub fn init() {
    IDT::new().load();
}

extern {
    fn idtload(pointer: *const IDT);

    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();

    fn irq0();
    fn irq1();
    fn irq2();
    fn irq3();
    fn irq4();
    fn irq5();
    fn irq6();
    fn irq7();
    fn irq8();
    fn irq9();
    fn irq10();
    fn irq11();
    fn irq12();
    fn irq13();
    fn irq14();
    fn irq15();
}
