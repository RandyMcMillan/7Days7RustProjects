use std::fs;
use std::io::{Read, Write};

pub fn read_file(path: &std::path::Path) -> Result<Vec<u8>, std::io::Error> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn write_file(path: &std::path::Path, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path)?;
    file.write_all(data)?;
    Ok(())
}
