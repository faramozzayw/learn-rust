#[allow(unused_imports)]
use std::fmt;
use std::{
    fmt::Display, 
    clone::Clone,
	marker::Copy,
	fmt::Debug
};

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
	value: T,
	left: Link<T>,
	right: Link<T>,
}

#[allow(dead_code)]
impl<T> Node<T>
where T: Display + Clone + Copy
{
	pub fn new(value: T, left: Link<T>, right: Link<T>) -> Self {
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

	pub fn new_leaf(value: T) -> Link<T> {
		Node::new_option(value, None, None)
	}
}

impl<T> Node<T>
where T: Debug + Display + Clone + Copy + PartialOrd
{
	fn insert(&mut self, value: T) {
		if self.value == value {
			panic!("The value '{}' is already exist in tree", value);
		}

		let node = if value > self.value {
			&mut self.left
		} else {
			&mut self.right
		};

		match node {
			&mut Some(ref mut subnode) => subnode.insert(value),
			&mut None => *node = Node::new_leaf(value),
		}
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
where T: Default + Display + Clone + Copy
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

	// TODO: return enum Result!
	pub fn add(&mut self, value: T) -> &mut Self {
		match &mut self.root {
			Some(node) => node.insert(value),
			None => self.root = Node::new_leaf(value)
		}

		self
	}
}