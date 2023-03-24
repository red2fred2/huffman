mod huffman_tree;

use std::fs;
use anyhow::anyhow;

use huffman_tree::HuffmanTree;

const TEXT_FILE_PATH: &str = "2022_fall-eecs660-pa2-input.txt";

/// Returns a list of letter frequencies from the input string.
/// Assumes only valid ASCII characters encoded 0-255.
fn get_letter_frequencies(string: &String) -> anyhow::Result<Vec<f32>> {
    // Throw a tantrum if it isn't ASCII
    if !string.is_ascii() {
        return Err(anyhow!("Input is not all ASCII characters"));
    }

    // count up each letter in a table
    let mut table: Vec<f32> = vec![0.0; 255];

	for char in string.as_bytes() {
		let index = usize::try_from(*char)?;

        table[index] += 1.0;
	}

    // Sum it up
	let total: f32 = table.iter().sum();
	Ok(table.iter().map(|e| e / total).collect())
}

fn main() {
    let text = fs::read_to_string(TEXT_FILE_PATH)
        .expect("Failed to read file");

    let frequencies = get_letter_frequencies(&text)
        .unwrap();

    let map = (0u8..255).map(|c| {
        char::from(c)
    }).collect();

    let huff = HuffmanTree::new(&frequencies, &map).unwrap();

	// let message = "test";
	let message = text;
	let string: Vec<char> = message.chars().collect();
	let bits = huff.encode(string);
	// println!("{message} -> {bits:?}");
	let l = bits.len();
	println!("{l} bits");
}
