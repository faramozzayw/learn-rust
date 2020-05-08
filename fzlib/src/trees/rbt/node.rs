#[allow(unused_imports)]
use std::{
	fmt::Display, 
	clone::Clone,
	marker::Copy,
	fmt::Debug,
	cmp::{
		Ordering,
		Ord
	},
	result::Result,
};

use super::{ Link, Color };
use crate::trees::TreeNodeMark;

#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
	pub(crate) value: T,
	pub(crate) left: Link<T>,
	pub(crate) right: Link<T>,
	pub(crate) color: Color,
}

impl<T> TreeNodeMark for Node<T>
where T: 'static + Clone + Debug
{}

#[allow(dead_code)]
impl<T> Node<T>
where T: Display + Debug + Clone + Copy + Default
{
	pub fn new(value: T, left: Link<T>, right: Link<T>, color: Color) -> Self {
		Node {
			value,
			left,
			right,
			color,
		}
	}

	pub fn new_link(value: T, left: Link<T>, right: Link<T>, color: Color) -> Link<T> {
		let node = Node {
			value,
			left,
			right,
			color,
		};

		let node = Box::new(node);
		let node = Some(node);
		
		node
	}
}

#[allow(dead_code)]
impl<T> Node<T>
where T: Display + Debug + Clone + Copy + Default
{
	pub fn recoloring(&mut self) {
		match &self.color {
			Color::Black => self.color = Color::Red,
			Color::Red => self.color = Color::Black,
		}
	}
}

#[allow(dead_code, unused_variables)]
impl<T> Node<T> 
where T: Debug + Display + Clone + Copy + PartialOrd + Ord + Default
{
	pub(crate) fn insert(&mut self, value: T) -> Result<(), ()> {
		let node = match self.value.cmp(&value) {
			Ordering::Greater => &mut self.left,
			Ordering::Less => &mut self.right,
			Ordering::Equal => return Err(()),
		};

		match node {
			&mut Some(ref mut subnode) => subnode.insert(value),
			&mut None => {
				*node = Node::new_link(value, None, None, Color::Red);

				return Ok(())
			},
		}
	}

	pub(crate) fn delete(&mut self, value: T) {
		unimplemented!();
	}
}