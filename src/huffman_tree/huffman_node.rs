use std::rc::Rc;

#[derive(Clone, Debug)]
/// A node in a Huffman tree
pub struct Node {
	pub left_child: Option<Rc<Node>>,
	pub right_child: Option<Rc<Node>>,
	pub value: Option<char>,
}

impl Node {
	/// Constructs a new parent node
	///
	/// * `left` - The left child node
	/// * `right` - The right child node
	pub fn new_parent(left: Rc<Node>, right: Rc<Node>) -> Self {
		let left_child = Some(Rc::clone(&left));
		let right_child = Some(Rc::clone(&right));

		Node {left_child, right_child, value: None}
	}

	/// Constructs a new leaf node
	///
	/// * `character` - The character value of this leaf node
	pub fn new_leaf(character: char) -> Self {
		Node {
			left_child: None,
			right_child: None,
			value: Some(character)
		}
	}
}
