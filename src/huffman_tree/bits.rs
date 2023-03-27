#[derive(Clone)]
/// A collection of individual bits
pub struct Bits {
	collection: Vec<bool>
}

impl Bits {
	/// Constructs an empty collection of bits
	pub fn new() -> Self {
		Bits {collection: Vec::new()}
	}

	/// Adds a single bit to the end of this collection
	pub fn add(&mut self, bit: bool) {
		self.collection.push(bit);
	}

	/// Append another Bits object to this one
	pub fn append(&mut self, other: &Self) {
		for bit in other.collection.iter() {
			self.add(*bit)
		}
	}

	/// Get the number of bits in the collection
	pub fn len(&self) -> usize {
		self.collection.len()
	}
}

impl std::fmt::Debug for Bits {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl std::string::ToString for Bits {
    fn to_string(&self) -> String {
		self.collection.iter().map(|e| {
			match e {
				false => "0",
				true => "1"
			}
		}).collect()
    }
}

mod benchmarks {
	#[allow(unused)]
	use test::Bencher;
	#[allow(unused)]
	use test::black_box;

	// Append bits
	#[bench]
	fn bits_append(b: &mut Bencher) {
		let mut bits = super::Bits::new();
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(true);
		bits.add(false);
		bits.add(false);
		bits.add(true);
		bits.add(false);
		bits.add(false);

		let mut other = super::Bits::new();
		other.add(false);
		other.add(true);
		other.add(true);
		other.add(false);
		other.add(true);
		other.add(false);
		other.add(false);
		other.add(true);

		b.iter(|| bits.append(black_box(&mut other)));
	}

	// Add bits
	#[bench]
	fn bits_add(b: &mut Bencher) {
		let mut bits = super::Bits::new();
		b.iter(|| bits.add(black_box(true)));
	}
}
