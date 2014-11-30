use core::intrinsics::transmute;


extern {
    static kend: u32;  // kernel end
}


static mut paddress: u32 = 0;  // placement address


pub fn init() {
    unsafe {
        paddress = transmute(&kend);
    }
}


pub fn alloc(size: uint) -> u32 {
    let mut size = size as u32;

    if size & 0xFFF != 0 {  // align 4096
        size &= 0xFFFFF000;
        size += 0x1000;
    }

    unsafe {
        let ptr = paddress;
        paddress += size;

        ptr
    }
}

#[test]
fn test_kmalloc() {
    unsafe {
        paddress = 0;
    }

    let ptr0 = kmalloc(4096);
    let ptr1 = kmalloc(128);
    let ptr2 = kmalloc(1);

    assert_eq!(ptr0, 0);
    assert_eq!(ptr1, 4096);
    assert_eq!(ptr1, 8192);
}
