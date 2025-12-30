# Huffman Encoding/Decoding

A Rust implementation of Huffman encoding and decoding algorithm. This project provides a command-line tool to compress text files using variable-length binary codes based on character frequency.

## Overview

Huffman encoding is a lossless data compression algorithm that assigns variable-length binary codes to characters based on their frequency in the input. More frequent characters receive shorter codes, resulting in efficient compression.

## Features

- ✅ Character frequency counting
- ✅ Huffman tree construction (bottom-up approach)
- ✅ Encoding table generation
- ✅ Text encoding to binary representation
- ✅ Binary decoding back to original text
- ✅ Full round-trip encoding/decoding

## Requirements

- Rust 1.70+ (or latest stable)
- Cargo (comes with Rust)

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
3. Encode the file content
4. Decode the encoded content
5. Write the decoded output to `<input_file>.huff`

### Example

```bash
cargo run -- data/S.csv
```

This will create `data/S.csv.huff` containing the decoded output.

## Project Structure

```
huffman/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library interface
│   └── huffman/
│       ├── mod.rs           # Huffman struct and main logic
│       └── huffman_node.rs  # HuffmanNode tree implementation
├── data/                    # Sample data files
├── Cargo.toml
└── README.md
```

## How It Works

1. **Frequency Analysis**: Counts the frequency of each character in the input
2. **Tree Building**: Constructs a binary tree using a min-heap, combining nodes with lowest frequencies
3. **Code Generation**: Traverses the tree to generate binary codes (0 for left, 1 for right)
4. **Encoding**: Replaces each character with its corresponding binary code
5. **Decoding**: Reads the binary codes and traverses the tree to reconstruct the original text

## Testing

Run the test suite:

```bash
cargo test
```

The project includes comprehensive tests for:
- Character frequency counting
- Tree construction
- Encoding/decoding round-trips
- Edge cases (single character, empty input, etc.)

## Implementation Details

- Uses `BinaryHeap` for efficient min-heap operations
- Implements `Ord` trait for HuffmanNode to enable heap ordering
- Stores encoding table as `HashMap<String, u8>` (code → character)
- Handles edge cases like single-character inputs
