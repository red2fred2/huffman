#![feature(test)]
extern crate test;

mod huffman_tree;

use huffman_tree::HuffmanTree;
use std::fs;

const TEXT_FILE_PATH: &str = "2022_fall-eecs660-pa2-input.txt";

fn main() {
	let text = fs::read_to_string(TEXT_FILE_PATH)
		.expect("Failed to read file");
	let tree = HuffmanTree::new(&text)
		.expect("Failed to build Huffman tree");
	let bits = tree.encode(&text)
		.expect("Failed to encode message");

	let num_bits = bits.len();
	println!("{num_bits} bits");
}

mod benchmarks {
	#[allow(unused)]
	use test::Bencher;

	// Full thing
	#[bench]
	fn main(b: &mut Bencher) {
		b.iter(|| super::main());
	}

	// Load file
	#[bench]
	fn load(b: &mut Bencher) {
		b.iter(|| {
			std::fs::read_to_string(super::TEXT_FILE_PATH)
				.expect("Failed to read file")
		});
	}
}
