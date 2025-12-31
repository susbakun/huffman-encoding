use std::path::PathBuf;
use std::fs;

mod huffman;
use huffman::Huffman;

fn read_file(file_path: PathBuf) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn write_file(file_path: PathBuf, contents: String) -> Result<(), std::io::Error> {
    fs::write(file_path, contents)
}

pub fn run() {
    let mut file_path = std::env::args()
        .skip(1)
        .next()
        .expect("Couldn't parse the argument");

    let input = read_file(PathBuf::from(&file_path))
        .expect("Failed to read the file");

    let mut huffman = Huffman::new(input);
    huffman.encode();

    let decoded = huffman.decode();

    file_path.push_str(".huff");

    write_file(PathBuf::from(&file_path), decoded)
        .expect("Couldn't write to the file");

}