pub fn get_hex_digits(n: &u16, d: u32, o: u32) -> usize {
    let base: u16 = 0x10;
    ((n / base.pow(o)) % base.pow(d)) as usize
}

pub fn is_bit_set(byte: &u8, n: u8) -> bool {
    if byte & (1 << n) == 0 {
        false
    } else {
        true
    }
}

pub fn get_bit(byte: &u8, n: u8) -> u8 {
    if is_bit_set(byte, n) {
        1
    } else {
        0
    }
}
