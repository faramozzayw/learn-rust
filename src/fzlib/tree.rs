#[allow(unused_imports)]
use std::fmt;
use std::{
    fmt::Display, 
    clone::Clone,
    marker::Copy,
};

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
	value: T,
	left: Link<T>,
	right: Link<T>,
}

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
		let root = Box::new(
			Node::new(root, None, None)
		);

		let root = Some(root);

		Tree { root }
	}
}

impl<T> Default for Tree<T>
where T: Default + Display + Clone + Copy
{
	fn default() -> Self {
		let root: T = Default::default(); 
		let root = Box::new(
			Node::new(root, None, None)
		);

		let root = Some(root);
		Tree { root }
	}
}

#[allow(dead_code)]
impl<T> Tree<T> 
where T: Display + Clone + Copy
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
				let node = Box::new(
					Node::new(value, None, None)
				);
		
				let node = Some(node);
				
				self.root = node;
			},
			_ => unimplemented!()
		}

		
		self
	}
}