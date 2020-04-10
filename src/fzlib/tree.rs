#[allow(unused_imports)]
use std::fmt;
use std::{
    fmt::Display, 
    clone::Clone,
	marker::Copy,
	fmt::Debug
};

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
	pub value: T,
	pub left: Link<T>,
	pub right: Link<T>,
}

#[allow(dead_code)]
impl<T> Node<T>
where T: Display + Clone + Copy
{
	fn new(value: T, left: Link<T>, right: Link<T>) -> Self {
		Node {
			value,
			left,
			right
		}
	}

	fn new_option(value: T, left: Link<T>, right: Link<T>) -> Link<T> {
		let node = Node {
			value,
			left,
			right
		};

		let node = Box::new(node);
		let node = Some(node);
		
		node
	}

	fn new_leaf(value: T) -> Link<T> {
		Node::new_option(value, None, None)
	}
}

#[derive(Debug)]
pub struct Tree<T> {
	root: Link<T>
}

#[allow(dead_code)]
impl<T> Tree<T> 
where T: Display + Clone + Copy
{
	pub fn new(root: T) -> Self {
		let root = Node::new_leaf(root);

		Tree { root }
	}
}

impl<T> Default for Tree<T>
where T: Default + Display + Clone + Copy + PartialOrd
{
	fn default() -> Self {
		let root: T = Default::default(); 
		let root = Node::new_leaf(root);
		
		Tree { root }
	}
}

#[allow(dead_code)]
impl<T> Tree<T> 
where T: Debug + Display + Clone + Copy + PartialOrd
{
	pub fn is_empty(&self) -> bool {
		match &self.root {
			None => true,
			_ => false,
		}
	}

	pub fn add(&mut self, value: T) -> &mut Self {
		match &self.root {
			None => {
				self.root = Node::new_leaf(value);
			},
			_ => self.add_to(value)
		}

		self
	}

	#[allow(unused_mut, unused_variables)]
	fn add_to(&mut self, value: T) {
		let current = &mut self.root;

		current.as_mut().map(|node| {
			let leaf = Node::new_leaf(value);

			if node.value < value {
				node.left = leaf;
			} else {
				node.right = leaf;
			}
		});
	}
}