#![allow(dead_code)]
use std::path::PathBuf;

mod huffman;
use huffman::Huffman;
mod io;
use io::*;


pub fn run() {
    let mut file_path = std::env::args()
        .skip(1)
        .next()
        .expect("Couldn't parse the argument");

    let input = read_string_file(PathBuf::from(&file_path))
        .expect("Failed to read the file");
    
    let mut huffman = Huffman::new(&input);
    let encoded = huffman.encode();

    println!("Original size: {} bytes", input.len());
    println!("Encoded size: {} bytes", encoded.len() / 8);

    file_path.push_str(".huff");

    write_bits_to_file(PathBuf::from(&file_path), &encoded)
        .expect("Couldn't write to the file");

}