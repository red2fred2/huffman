use std::{sync::Arc, collections::HashMap};
use anyhow::anyhow;
use bit_vec::BitVec;

pub struct HuffmanTree<T> {
    lookup_table: HashMap<T, BitVec>,
}

impl<T: Copy + core::fmt::Debug> HuffmanTree<T> {
    /// Builds a Huffman tree based on a list of frequencies, and a map of what
    /// symbol is represented by each place in the list.
    pub fn new(frequencies: &Vec<f32>, map: &Vec<T>) -> anyhow::Result<Self> {
        let mut table = build_table(frequencies, map)?;
        // println!("{table:#?}");

        while table.len() > 1 {
            // Do the witchcraft
            let (left_probability, left_node) = table.remove(0);
            let (right_probability, right_node) = table.remove(0);

            let left_child = Some(Arc::clone(&left_node));
            let right_child = Some(Arc::clone(&right_node));

            let node = Arc::new(Node {is_leaf: false, left_child, right_child, value: None});

            let probability = left_probability + right_probability;

            match table.binary_search_by(|e| {
                f32_nans_last(e.0, probability)
            }) {
                Ok(pos) => table.insert(pos, (probability, node)),
                Err(pos) => table.insert(pos, (probability, node)),
            };
        }

        let tree = Arc::clone(&table[0].1);
        let lookup_table = generate_lookup_table(tree);
        Ok(HuffmanTree{lookup_table})
    }

}

#[derive(Debug)]
struct Node<T> {
    is_leaf: bool,
    left_child: Option<Arc<Node<T>>>,
    right_child: Option<Arc<Node<T>>>,
    value: Option<T>,
}

fn build_table<T>(frequencies: &Vec<f32>, map: &Vec<T>) -> anyhow::Result<Vec<(f32, Arc<Node<T>>)>>
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

            (p, Arc::new(node))
        })
        .collect()
    )
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

fn generate_lookup_table<T>(tree: Arc<Node<T>>) -> HashMap<T, BitVec> {
    let mut table = HashMap::new();



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
