use core::mem::size_of;
use core::prelude::*;


const GDT_SIZE: uint = 5;

type GDTTable = [GDTEntry, ..GDT_SIZE];


#[repr(packed)]
struct GDTEntry {
    limit_low: u16,  // the lower 16 bits of the limit
    base_low: u16,  // the lower 16 bits of the base
    base_middle: u8,  // the next 8 bits of the base
    access: u8,  // access flags, determine what ring this segment can be used in
    granularity: u8,
    base_high: u8
}

impl GDTEntry {

    pub fn new(base: u32, limit: uint, access: u8, granularity: u8) -> GDTEntry {
        GDTEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: (base >> 16 & 0xFF) as u8,
            access: access,
            granularity: ((limit >> 16 & 0x0F) as u8) | granularity & 0xF0,
            base_high: (base >> 24 & 0xFF) as u8,
        }
    }

    pub fn flat(access: u8, granularity: u8) -> GDTEntry {
        GDTEntry::new(0, 0xFFFFFFFF, access, granularity)
    }

    pub fn null() -> GDTEntry {
        GDTEntry::new(0, 0, 0, 0)
    }
}

#[repr(packed)]
struct GDT {
    limit: u16,
    base: *const GDTTable,
}

impl GDT {

    pub fn new() -> GDT {

        unsafe {
            TABLE = [GDTEntry::null(),  // null segment
                     GDTEntry::flat(0x9A, 0xCF),  // code segment
                     GDTEntry::flat(0x92, 0xCF),  // data segment
                     GDTEntry::flat(0xFA, 0xCF),  // user mode code segment
                     GDTEntry::flat(0xF2, 0xCF)];  // user mode data segment

            GDT {
                limit: (size_of::<GDTEntry>() * TABLE.len() - 1) as u16,
                base: &TABLE as *const GDTTable,
            }
        }
    }

    pub fn load(&self) {
        unsafe {
            gdtload(self as *const GDT);
        }
    }
}


static mut TABLE: GDTTable = [
    GDTEntry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0
    }, ..GDT_SIZE
];

pub fn init() {
    GDT::new().load();
}

extern {
    fn gdtload(pointer: *const GDT);
}
