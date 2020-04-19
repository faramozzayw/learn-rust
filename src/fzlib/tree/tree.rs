use std::{
    fmt::Display, 
    clone::Clone,
	marker::Copy,
	fmt::Debug,
};

use super::{ Link, Node };

#[derive(Debug)]
pub struct Tree<T> {
	pub(crate) root: Link<T>
}

#[allow(dead_code)]
impl<T> Tree<T> 
where T: Debug + Display  + Clone + Copy + Default
{
	pub fn new(root: T) -> Self {
		let root = Node::new_leaf(root);

		Tree { root }
	}
}

impl<T> Default for Tree<T>
where T: Default + Debug + Display  + Clone + Copy
{
	fn default() -> Self {
		let root: T = Default::default(); 
		let root = Node::new_leaf(root);
		
		Tree { root }
	}
}

#[allow(dead_code)]
impl<T> Tree<T> 
where T: Debug + Display + Clone + Copy + PartialOrd + Ord + Default
{
	pub fn is_empty(&self) -> bool {
		self.root.is_none()
	}

	// TODO: return enum Result!
	pub fn add(&mut self, value: T) -> &mut Self {
		match &mut self.root {
			Some(node) => node.insert(value),
			None => self.root = Node::new_leaf(value)
		}

		self
	}

	// TODO: return enum Result!
	pub fn delete(&mut self, value: T) {
		if self.is_empty() {
			panic!("Tree is empty!");
		} else if let Some(ref mut node) = &mut self.root {
			if node.value == value && node.is_leaf() {
				self.root = None
			} else {
				node.delete(value)
			}
		}
	}
}