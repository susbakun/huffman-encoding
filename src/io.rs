use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::{self, File};

use bitvec::vec::BitVec;


pub fn read_string_file(file_path: PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

pub fn write_string_to_file(file_path: PathBuf, contents: &String) -> Result<(), std::io::Error> {
    fs::write(file_path, contents)
}

pub fn read_huffman_file(file_path: PathBuf) -> Result<BitVec<u8>, std::io::Error> {
    let mut file = File::open(file_path)?;

    let mut count_bytes = [0u8; 8];
    file.read_exact(&mut count_bytes)?;
    let bit_count = u64::from_le_bytes(count_bytes) as usize;

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut bitvec = BitVec::from_vec(bytes);
    bitvec.truncate(bit_count);

    Ok(bitvec)
}

pub fn write_bits_to_file(file_path: PathBuf, contents: &BitVec) -> Result<(), std::io::Error> {
    let mut bytes = Vec::new();
    let mut file = File::create(file_path)?;

    let bit_count = contents.len();
    file.write_all(&bit_count.to_le_bytes())?;
    
    for chunk in contents.chunks(8) {
        let mut byte = 0u8;
        for (i, bit) in chunk.iter().enumerate() {
            if *bit {
                byte |= 1 << i;
            }
        }
        bytes.push(byte);
    }

    file.write_all(&bytes)
}