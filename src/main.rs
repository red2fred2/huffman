#![feature(test)]
extern crate test;

mod huffman_tree;

use huffman_tree::HuffmanTree;
use std::{fs, /*time::Instant*/};

const TEXT_FILE_PATH: &str = "2022_fall-eecs660-pa2-input.txt";

fn main() {
	let text = fs::read_to_string(TEXT_FILE_PATH)
		.expect("Failed to read file");

	// let timer = Instant::now();

	let tree = HuffmanTree::new(&text)
		.expect("Failed to build Huffman tree");
	let bits = tree.encode(&text)
		.expect("Failed to encode message");

	let num_bits = bits.len();

	// let time = timer.elapsed().as_micros();
	println!("{num_bits} bits");
	// println!("time: {time}us")
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
