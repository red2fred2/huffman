use std::{rc::Rc, collections::HashMap};
use anyhow::anyhow;
use bit_vec::BitVec;

pub struct HuffmanTree<T> {
    lookup_table: HashMap<T, Bits>,
}

impl<T: Copy + std::cmp::Eq + std::hash::Hash + core::fmt::Debug> HuffmanTree<T> {
    /// Builds a Huffman tree based on a list of frequencies, and a map of what
    /// symbol is represented by each place in the list.
    pub fn new(frequencies: &Vec<f32>, map: &Vec<T>) -> anyhow::Result<Self> {
        let mut table = build_table(frequencies, map)?;

        while table.len() > 1 {
            // Do the witchcraft
            let (left_probability, left_node) = table.remove(0);
            let (right_probability, right_node) = table.remove(0);

            let left_child = Some(Rc::clone(&left_node));
            let right_child = Some(Rc::clone(&right_node));

            let node = Rc::new(Node {is_leaf: false, left_child, right_child, value: None});

            let probability = left_probability + right_probability;

            match table.binary_search_by(|e| {
                f32_nans_last(e.0, probability)
            }) {
                Ok(pos) => table.insert(pos, (probability, node)),
                Err(pos) => table.insert(pos, (probability, node)),
            };
        }

        let tree = Rc::clone(&table[0].1);
        let lookup_table = generate_lookup_table(tree);

		println!("{lookup_table:#?}");
        Ok(HuffmanTree{lookup_table})
    }

	pub fn encode(&self, str: Vec<T>) -> Bits {
		let mut bits = Bits::new();

		for c in str {
			let encoded = self.lookup_table
				.get(&c)
				.expect("Character not found in encoding table");
			bits.append(encoded);
		}

		bits
	}

}

#[derive(Clone)]
pub struct Bits {
	length: usize,
	collection: BitVec,
}

impl Bits {
	pub fn new() -> Self {
		Bits {
			length: 0,
			collection: BitVec::new(),
		}
	}

	pub fn add(&mut self, bit: u8) {
		let len = self.len();

		if self.collection.len() == len {
			let mut other = BitVec::from_bytes(&[0x0]);
			self.collection.append(&mut other);
		}

		self.collection.set(len, bit & 0x01 == 0x01);
		self.length += 1;
	}

	/// I'm getting tired of bit level stuff, I'm just going to do this naively
	pub fn append(&mut self, other: &Self) {
		other.get().chars().for_each(|c| self.add(ctob(c)));
	}

	pub fn len(&self) -> usize {
		self.length
	}

	pub fn get(&self) -> String {
		let mut str = String::new();

		for i in 0..self.len() {
			let bit = self.collection.get(i).expect("BitVec freaked out");
			str += if bit {"1"} else {"0"};
		}

		str
	}
}

impl std::fmt::Debug for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let bits = self.get();
		write!(f, "{bits}")
    }
}

#[derive(Clone, Debug)]
struct Node<T>
where T: Clone {
    is_leaf: bool,
    left_child: Option<Rc<Node<T>>>,
    right_child: Option<Rc<Node<T>>>,
    value: Option<T>,
}

fn build_table<T>(frequencies: &Vec<f32>, map: &Vec<T>) -> anyhow::Result<Vec<(f32, Rc<Node<T>>)>>
where T: Copy {
    // Load table
    let mut table = zip(frequencies, map)?;

    // Sort it
    table.sort_unstable_by(|a, b| {
        f32_nans_last(a.0, b.0)
    });

    // Filter out unused
    Ok(
        table.iter()
        .map(|e| *e)
        .filter(|e| e.0 > 0.0)
        .map(|(p, v)| {
            let node = Node {
                is_leaf: true,
                left_child: None,
                right_child: None,
                value: Some(v)
            };

            (p, Rc::new(node))
        })
        .collect()
    )
}

fn ctob(c: char) -> u8 {
	match c {
		'0' => 0x0,
		'1' => 0x1,
		_ => panic!()
	}
}

/// f32 ordering function since testing if floats are equivalent is a nightmare
/// Stolen from https://users.rust-lang.org/t/sorting-a-vec-of-f32-without-ever-panic/37540/2
fn f32_nans_last(a: f32, b: f32) -> std::cmp::Ordering {
    match a.partial_cmp(&b) {
        Some(ord) => ord,
        None => match (a.is_nan(), b.is_nan()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, _) => std::cmp::Ordering::Greater,
            (_, true) => std::cmp::Ordering::Less,
            (_, _) => std::cmp::Ordering::Equal, // should never happen
        },
    }
}

fn generate_lookup_table<T>(tree: Rc<Node<T>>) -> HashMap<T, Bits>
where T: Clone + std::cmp::Eq + std::hash::Hash {
    let mut table = HashMap::new();

	fn traverse<T>(subtree: Rc<Node<T>>, table: &mut HashMap<T, Bits>, code: Bits)
	where T: Clone + std::cmp::Eq + std::hash::Hash {
		let subtree = (*subtree).clone();

		if subtree.is_leaf {
			// Slap that value in the table
			let value = subtree.value.expect("Node marked as leaf with no value");
			table.insert(value, code);
		} else {
			// Traverse children
			match subtree.left_child {
				Some(node) => {
					let mut code = code.clone();
					code.add(0);

					traverse(node, table, code)
				},
				_ => ()
			};

			match subtree.right_child {
				Some(node) => {
					let mut code = code.clone();
					code.add(1);

					traverse(node, table, code)
				},
				_ => ()
			};
		}
	}

	traverse(tree, &mut table, Bits::new());
    table
}

/// Zips two Vecs together in a nicer way than standard rust
fn zip<A, B>(a: &Vec<A>, b: &Vec<B>) -> anyhow::Result<Vec<(A, B)>>
where A: Copy, B: Copy {
    if a.len() != b.len() {
        return Err(anyhow!("Vectors are not the same size"));
    }

    let mut zipper = Vec::new();

    for i in 0..a.len() {
        zipper.push((a[i], b[i]));
    }

    Ok(zipper)
}
