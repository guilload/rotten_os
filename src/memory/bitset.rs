struct BitSet {
    bytes: [uint, ..1024 / 32],
}

impl BitSet {

    fn new() -> BitSet {
        BitSet {
            bytes: [0, ..1024 / 32]
        }
    }

    fn index(&self, value: u32) -> uint {
        (value >> 5) as uint  // divide by 32
    }

    fn mask(&self, value: u32) -> uint {
        let offset = (value & 0x1F) as uint;  // mod 32
        0x1 << offset
    }

    fn clear(&mut self, value: u32) {
        let index = self.index(value);
        let mask = self.mask(value);
        self.bytes[index] &= !mask;
    }

    fn present(&self, value: u32) -> bool {
        let index = self.index(value);
        let mask = self.mask(value);
        self.bytes[index] & mask != 0
    }

    fn set(&mut self, value: u32) {
        let index = self.index(value);
        let mask = self.mask(value);
        self.bytes[index] |= mask;
    }

    fn first(&self) -> Option<uint> {
        for (i, byte) in self.bytes.iter().enumerate() {

            if *byte == 0xFFFFFFFF {
                continue
            }

            let mut mask = 0x1;

            for j in range(0, 32) {

                if *byte & mask == 0 {
                    return Some(i * 32 + j);
                }

                mask <<= 1;
            }
        }
        None
    }
}

#[test]
fn test_bitset() {
    let mut bitset = BitSet::new();

    assert_eq!(bitset.first().unwrap(), 0);

    bitset.set(0);
    assert!(bitset.present(0));
    assert_eq!(bitset.first().unwrap(), 1);

    bitset.clear(0);
    assert!(!bitset.present(0));
    assert_eq!(bitset.first().unwrap(), 0);

    for i in range(0, 42) {
        bitset.set(i);
        assert!(bitset.present(i));
    }

    assert_eq!(bitset.first().unwrap(), 42);

    bitset.clear(32);
    assert_eq!(bitset.first().unwrap(), 32);
}
