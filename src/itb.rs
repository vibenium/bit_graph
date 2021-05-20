pub fn int_to_bit8(num: &u8) -> String {
    let mut mask: u8 = 0x80; // 1000 0000
    let mut result: u8 = 0x00;  
    let mut string = String::with_capacity(8);
    for i in 0..8 {
        result = mask & num; // isolating bits
        if result == 0 {
            string.push_str("0");
        } else { // result == 1
            string.push_str("1");
        }
        mask = mask >> 1;
    }
    return string;
}
pub fn int_to_bit16(num: &u16) -> String {
    let mut mask: u16 = 0x8000; // 1000 0000 0000 0000
    let mut result: u16 = 0x0000;
    let mut string = String::with_capacity(16);
    for i in 0..16 {
        result = mask & num; // isolating bits
        if result == 0 {
            string.push_str("0");
        } else { // result == 1
            string.push_str("1");
        }
        mask = mask >> 1;
    }
    return string;
}
pub fn int_to_bit32(num: &u32) -> String {
    let mut mask: u32 = 0x80000000; // 1000 0000 0000 0000 0000 0000 0000 0000
    let mut result: u32 = 0x00000000;
    let mut string = String::with_capacity(32);
    for i in 0..32 {
        result = mask & num; // isolating bits
        if result == 0 {
            string.push_str("0");
        } else { // result == 1
            string.push_str("1");
        }
        mask = mask >> 1;
    }
    return string;
}
pub fn int_to_bit64(num: &u64) -> String {
    let mut mask: u64 = 0x8000000000000000;
    let mut result: u64 = 0x0000000000000000;
    let mut string = String::with_capacity(64);
    for i in 0..64 {
        result = mask & num; // isolating bits
        if result == 0 {
            string.push_str("0");
        } else { // result == 1
            string.push_str("1");
        }
        mask = mask >> 1;
    }
    return string;
}