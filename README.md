# Huffman Encoding/Decoding

A Rust implementation of Huffman encoding and decoding algorithm. This project provides a command-line tool to compress text files using variable-length binary codes based on character frequency.

## Overview

Huffman encoding is a lossless data compression algorithm that assigns variable-length binary codes to characters based on their frequency in the input. More frequent characters receive shorter codes, resulting in efficient compression.

## Features

- ✅ Character frequency counting
- ✅ Huffman tree construction (bottom-up approach using min-heap)
- ✅ Encoding table generation with `BitVec` for efficient bit manipulation
- ✅ Text encoding to binary representation
- ✅ Binary decoding back to original text
- ✅ Full round-trip encoding/decoding
- ✅ Edge case handling (empty input, single character)
- ✅ Binary file I/O with bit-count header for accurate reconstruction
- ✅ Compression statistics output (original vs encoded size)

## Requirements

- Rust 2024 edition
- Cargo (comes with Rust)

## Dependencies

- `bitvec` 1.0.1 - Efficient bit-level operations

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd huffman
cargo build --release
```

## Usage

### Command Line

Run the program with a file path as an argument:

```bash
cargo run -- <input_file>
```

Or use the compiled binary:

```bash
./target/release/huffman <input_file>
```

The program will:
1. Read the input file
2. Build a Huffman tree based on character frequencies
3. Encode the file content to a `BitVec`
4. Display compression statistics (original vs encoded size)
5. Write the encoded binary to `<input_file>.huff` with a bit-count header

### Example

```bash
cargo run -- data/S.csv
```

This will create `data/S.csv.huff` containing the decoded output.

### Library Usage

```rust
use huffman::Huffman;

let input = "hello world".to_string();
let mut huffman = Huffman::new(input);

// Encode to BitVec
let encoded = huffman.encode();

// Decode back to string
let decoded = huffman.decode();
```

## Project Structure

```
huffman/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library interface and run function
│   ├── io.rs                # File I/O utilities (read/write text and binary)
│   └── huffman/
│       ├── mod.rs           # Huffman struct (encode/decode/table generation)
│       └── huffman_node.rs  # HuffmanNode tree and min-heap construction
├── data/                    # Sample data files
├── Cargo.toml
└── README.md
```

## How It Works

1. **Frequency Analysis**: Counts byte frequency using `HashMap<u8, usize>`
2. **Min-Heap Building**: Creates `BinaryHeap<HuffmanNode>` ordered by frequency (ascending)
3. **Tree Construction**: Repeatedly combines two lowest-frequency nodes until one root remains
4. **Code Generation**: Traverses tree recursively - left = 0, right = 1 - storing codes in `HashMap<u8, BitVec>`
5. **Encoding**: Maps each input byte to its `BitVec` code
6. **Decoding**: Iterates through bits, matching accumulated bits against the code table

## Testing

Run the test suite:

```bash
cargo test
```

The project includes 12 tests covering:
- Character frequency counting
- Tree construction (including single-node trees)
- Encoding/decoding round-trips
- Edge cases (empty input, single character, all unique characters)
- Table integrity (no empty codes, all bytes present)
- Deterministic encoding

## Implementation Details

- Uses `BinaryHeap` with custom `Ord` implementation for min-heap behavior
- `BitVec` from the `bitvec` crate for memory-efficient bit storage
- Encoding table stored as `HashMap<u8, BitVec>` (byte → code)
- Single-character inputs receive a default code of `[0]`
- Empty inputs handled gracefully with empty tree/output

### I/O Module (`io.rs`)

Provides four utility functions:
- `read_string_file()` - Read text file to `String`
- `write_string_to_file()` - Write `String` to text file
- `read_huffman_file()` - Read `.huff` binary file, extracting bit count header and reconstructing `BitVec`
- `write_bits_to_file()` - Write `BitVec` to binary file with 8-byte little-endian bit count header
