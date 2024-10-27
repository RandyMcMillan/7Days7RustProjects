pub fn encrypt_decrypt(data: &mut [u8], key: u8) {
    data.iter_mut().for_each(|byte| *byte ^= key);
}

pub fn get_key() -> u8 {
    42 // A simple key for demonstration
}
