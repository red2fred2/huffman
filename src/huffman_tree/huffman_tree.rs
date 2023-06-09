use anyhow::{anyhow, Result, Context};
use fnv::{FnvHasher, FnvHashMap};
use std::{rc::Rc, collections::HashMap, hash::BuildHasherDefault};
use super::bits::Bits;
use super::huffman_node::Node;

type NodeTable = Vec<(usize, Rc<Node>)>;

pub struct HuffmanTree {
	lookup_table: Vec<Option<Bits>>
}

impl HuffmanTree {
	/// Construct a new Huffman tree from example text
	pub fn new(example_text: &String) -> Result<Self> {
		let frequencies = get_letter_frequencies(&example_text);
		let mut table = Self::init_table(frequencies);

		while table.len() > 1 {
			Self::build_tree(&mut table)?;
		}

		let tree = Rc::clone(&table[0].1);
		let mut lookup_table = FnvHashMap::default();
		Self::traverse(&tree, &mut lookup_table, &Bits::new());

		let lookup_table = Self::convert_table(lookup_table);
		Ok(HuffmanTree{lookup_table})
	}

	/// Runs one step of building the tree
	///
	/// This will error when there are less than two nodes to combine.
	fn build_tree(table: &mut NodeTable) -> Result<()> {
		if table.len() < 2 {
			return Err(anyhow!("Attempt to build tree without two nodes to combine"));
		}

		let (n1, left) = table.remove(0);
		let (n2, right) = table.remove(0);

		let num = n1 + n2;
		let node = Rc::new(Node::new_parent(left, right));

		Self::insert_table_entry(table, num, node);
		Ok(())
	}

	/// Converts the hashmap table to a Vec
	///
	/// Since there are only a few hundred options, this sacrifices a bit of
	/// space to avoid running the hash function.
	fn convert_table(table: HashMap<char, Bits, BuildHasherDefault<FnvHasher>>) -> Vec<Option<Bits>> {
		let (max_char, _) = table.iter()
			.max_by_key(|(c, _)| u32::from(**c))
			.unwrap();

		// Initialize table
		let mut output_table = Vec::new();
		for _ in 0..u32::from(*max_char) + 1 {
			output_table.push(None);
		}

		table.iter().for_each(|(character, bits)| {
			let index = *character as usize;
			output_table[index] = Some(bits.clone());
		});

		output_table
	}

	/// Encodes a string to a Bits object using this Huffman tree
	///
	/// This uses the lookup table learned from the example text provided at
	/// construction. This function will fail if any characters being encoded
	/// were not in that initial text.
	pub fn encode(&self, string: &String) -> Result<Bits> {
		let mut encoded_string = Bits::new();

		for character in string.chars() {
			let encoded_character = self.encode_character(&character)?;
			encoded_string.append(encoded_character);
		}

		Ok(encoded_string)
	}

	/// Encodes a single character to a Bits object using this Huffman tree
	///
	/// This uses the lookup table learned from the example text provided at
	/// construction. This function will fail if the character being encoded was
	/// not in that initial text.
	fn encode_character(&self, character: &char) -> Result<&Bits> {
		let error_message = "Character not found in lookup table";

		let index = *character as usize;
		let result = self.lookup_table.get(index);

		match result {
			Some(res) => res.as_ref().context(error_message),
			None => Err(anyhow!(error_message))
		}
	}

	/// Traverse a subtree and extract its data into a lookup table
	fn traverse(subtree: &Rc<Node>, table: &mut HashMap<char, Bits, BuildHasherDefault<FnvHasher>>, code: &Bits) {
		if let Some(value) = subtree.value {
			table.insert(value, code.clone());
		} else {
			Self::traverse_child(&subtree.left_child, table, code, false);
			Self::traverse_child(&subtree.right_child, table, code, true);
		}
	}

	/// Traverses down a child node in a subtree
	fn traverse_child(child: &Option<Rc<Node>>, table: &mut HashMap<char, Bits, BuildHasherDefault<FnvHasher>>, code: &Bits, new_bit: bool) {
		if let Some(node) = child {
			let mut code = code.clone();
			code.add(new_bit);

			Self::traverse(&node, table, &code);
		}
	}


	/// Initializes a sorted table of leaf nodes from a hash map of character frequencies
	fn init_table(frequencies: HashMap<char, usize, BuildHasherDefault<FnvHasher>>) -> NodeTable {
		let mut table: Vec<(char, usize)> = frequencies.into_iter().collect();
		table.sort_unstable_by_key(|e| e.1);

		table.iter()
			.map(|(character, num)| {
				let node = Node::new_leaf(*character);
				(*num, Rc::new(node))
			})
			.collect()
	}

	/// Finds a spot to insert an entry into a node table
	fn insert_table_entry(table: &mut NodeTable, num: usize, node: Rc<Node>) {
		let position = table.binary_search_by_key(&num, |(n, _)| *n);

		match position {
			Ok(pos) => table.insert(pos, (num, node)),
			Err(pos) => table.insert(pos, (num, node)),
		}
	}
}

/// Generates a list of letter frequencies
///
/// Returns a map of characters and the number of times they appear
fn get_letter_frequencies(string: &String) -> HashMap<char, usize, BuildHasherDefault<FnvHasher>> {
	let mut frequencies = FnvHashMap::with_capacity_and_hasher(100, Default::default());

	for character in string.chars() {
		let entry = frequencies.get_mut(&character);

		match entry {
			Some(e) => *e += 1,
			None => {frequencies.insert(character, 1);}
		};
	}

	frequencies
}

mod benchmarks {

	#[allow(unused)]
	use test::Bencher;
	#[allow(unused)]
	use test::black_box;
	#[allow(unused)]
	use super::HuffmanTree;

	// Construct a new tree
	#[bench]
	fn new(b: &mut Bencher) {
		let text = std::fs::read_to_string("2022_fall-eecs660-pa2-input.txt")
			.expect("Failed to read file");

		b.iter(|| HuffmanTree::new(&text));
	}

	// Encode a string
	#[bench]
	fn encode(b: &mut Bencher) {
		let text = std::fs::read_to_string("2022_fall-eecs660-pa2-input.txt")
			.expect("Failed to read file");
		let tree = super::HuffmanTree::new(&text)
			.expect("Failed to build Huffman tree");

		b.iter(|| tree.encode(&text));
	}

	// Encode a character
	#[bench]
	fn encode_character(b: &mut Bencher) {
		let text = std::fs::read_to_string("2022_fall-eecs660-pa2-input.txt")
			.expect("Failed to read file");
		let tree = super::HuffmanTree::new(&text)
			.expect("Failed to build Huffman tree");

		b.iter(|| tree.encode_character(black_box(&'c')));
	}

	// Get letter frequencies
	#[bench]
	fn get_letter_frequencies(b: &mut Bencher) {
		let text = std::fs::read_to_string("2022_fall-eecs660-pa2-input.txt")
			.expect("Failed to read file");

		b.iter(|| super::get_letter_frequencies(&text));
	}

	// Lookup bits from table
	#[bench]
	fn tree_lookup(b: &mut Bencher) {
		let text = std::fs::read_to_string("2022_fall-eecs660-pa2-input.txt")
			.expect("Failed to read file");
		let tree = super::HuffmanTree::new(&text)
			.expect("Failed to build Huffman tree");

		b.iter(|| {
			let index = 'Q' as usize;
			let _ = tree.lookup_table.get(black_box(index));
		})
	}
}