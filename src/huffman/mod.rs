use bitvec::prelude::*;
use std::collections::HashMap;

mod huffman_node;
use huffman_node::HuffmanNode;


#[derive(Debug)]
pub struct Huffman {
    table: HashMap<u8, BitVec>,
    input: String,
    encoded: BitVec,
}

impl Huffman{
    pub fn new(input: &String) -> Self {
        let root = HuffmanNode::new(&input);
        let input = input.clone();
        let mut huffman = Huffman {
            table: HashMap::new(),
            input,
            encoded: bitvec![],
        };

        huffman.make_table(Box::new(root),
        &mut bitvec![]);

        huffman
    }

    pub fn make_table(&mut self, curr_node: Box<HuffmanNode>, 
        code: &mut BitVec) {
        if let Some(byte) = curr_node.byte {
            // Single character case: assign a code of [false] if code is empty
            if code.is_empty() {
                self.table.insert(byte, bitvec![0;1]);
            } else {
                self.table.insert(byte, code.clone());
            }

            return
        }
    
        if let Some(left) = curr_node.left {
            code.push(false);
            self.make_table(left, code);
            code.pop();
        }
    
        if let Some(right) = curr_node.right {
            code.push(true);
            self.make_table(right, code);
            code.pop();
        }
    }
    
    pub fn encode(&mut self) -> BitVec {
        let mut bits = bitvec![];

        self.input.bytes().for_each(|i_byte| {
            let code = self.table
                .get(&i_byte)
                .unwrap();
            
            bits.extend(code.iter());
        });

        self.encoded = bits.clone();
    
        bits
    }
    
    pub fn decode(&mut self, encoded: &BitVec) -> String {
        let mut current = bitvec![];
        let mut output = String::new();
    
        for i in 0..encoded.len() {
            let bit = encoded[i];
            current.push(bit);
        
            self.table
                .iter()
                .find(|entry| entry.1.eq(&current))
                .and_then(|entry| {
                    current.clear();
                    let char = *entry.0 as char;
                    output.push(char);
                    Some(())
                });
        }
    
        output
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let input = "hello world".to_string();
        let mut huffman = Huffman::new(&input.clone());

        let encoded = huffman.encode();
        assert!(!encoded.is_empty(), "encoded output should not be empty");

        let decoded = huffman.decode(&encoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn single_character_input() {
        let input = "aaaaaa".to_string();
        let mut huffman = Huffman::new(&input.clone());

        let encoded = huffman.encode();
        let decoded = huffman.decode(&encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn empty_input() {
        let input = "".to_string();
        let mut huffman = Huffman::new(&input.clone());

        let encoded = huffman.encode();
        let decoded = huffman.decode(&encoded);

        assert!(encoded.is_empty());
        assert_eq!(decoded, input);
    }

    #[test]
    fn all_unique_characters() {
        let input = "abcdefg".to_string();
        let mut huffman = Huffman::new(&input.clone());

        let encoded = huffman.encode();
        let decoded = huffman.decode(&encoded);

        assert_eq!(decoded, input);
    }

    #[test]
    fn table_contains_all_input_bytes() {
        let input = "mississippi".to_string();
        let huffman = Huffman::new(&input.clone());

        for byte in input.bytes() {
            assert!(
                huffman.table.contains_key(&byte),
                "missing Huffman code for byte {}",
                byte
            );
        }
    }

    #[test]
    fn no_empty_codes_in_table() {
        let input = "hello".to_string();
        let huffman = Huffman::new(&input);

        for (byte, code) in &huffman.table {
            assert!(
                !code.is_empty(),
                "byte {} has an empty Huffman code",
                byte
            );
        }
    }

    #[test]
    fn encoding_is_deterministic() {
        let input = "banana".to_string();

        let mut h1 = Huffman::new(&input.clone());
        let mut h2 = Huffman::new(&input.clone());

        let b1 = h1.encode();
        let b2 = h2.encode();

        assert_eq!(b1, b2);
    }

}
