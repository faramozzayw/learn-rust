use std::{
	fmt::Display, 
	clone::Clone,
	marker::Copy,
	fmt::Debug,
	cmp::{
		Ordering,
		Ord
	},
};

use super::{ Link };
use crate::trees::TreeNodeMark;

#[derive(Debug, Clone)]
pub(crate) struct Node<T> {
	pub(crate) value: T,
	pub(crate) left: Link<T>,
	pub(crate) right: Link<T>,
}

impl<T> TreeNodeMark for Node<T>
where T: 'static + Clone + Debug
{}

#[allow(dead_code)]
impl<T> Node<T>
where T: Display + Debug + Clone + Copy + Default
{
	pub fn new(value: T, left: Link<T>, right: Link<T>) -> Self {
		Node {
			value,
			left,
			right,
		}
	}

	fn new_link(value: T, left: Link<T>, right: Link<T>) -> Link<T> {
		let node = Node {
			value,
			left,
			right,
		};

		let node = Box::new(node);
		let node = Some(node);
		
		node
	}

	pub fn new_leaf(value: T) -> Link<T> {
		Node::new_link(value, None, None)
	}
}

#[allow(dead_code)]
impl<T> Node<T>
where T: Debug + Display + Clone + Copy + PartialOrd
{
	pub(crate) fn is_leaf(&self) -> bool {
		match self.children_count() {
			0 => true,
			_ => false
		}
	}

	pub(crate) fn children_count(&self) -> usize {
		let mut count = 0;

		if let Some(_) = &self.left {
			count += 1;
		}

		if let Some(_) = &self.right {
			count += 1;
		}

		count
	}

	pub(crate) fn min_node(&self) -> &Node<T> {
		let mut current_node = &*self;

		while let Some(node) = &current_node.left {
			current_node = &node;
		}

		current_node
	}

	fn min_right(&self) -> &Node<T> {
		if let Some(right) = &self.right {
			right.min_node()
		} else {
			&self.min_node()
		}
	}
}

impl<T> Node<T>
where T: Debug + Display + Clone + Copy + PartialOrd + Ord + Default
{
	pub(crate) fn insert(&mut self, value: T) {
		let node = match self.value.cmp(&value) {
			Ordering::Greater => &mut self.left,
			Ordering::Less => &mut self.right,
			Ordering::Equal => panic!("The value '{}' is already exist in tree", value),
		};

		match node {
			&mut Some(ref mut subnode) => subnode.insert(value),
			&mut None => *node = Node::new_leaf(value),
		}
	}

	pub(crate) fn delete(&mut self, value: T) {
		let mut tmp_self = Some(Box::new(self.clone()));
		let mut is_root = false;

		let node = match self.value.cmp(&value) {
			Ordering::Greater => &mut self.left,
			Ordering::Less => &mut self.right,
			Ordering::Equal => {
				is_root = true;
				&mut tmp_self
			},
		};

		match node {
			&mut Some(ref mut subnode) if subnode.value == value => {
				match subnode.children_count() {
					0 => *node = None,
					1 => {
						*node = if let Some(_) = &subnode.left {
							subnode.left.take()
						} else {
							subnode.right.take()
						};
					},
					2 => {
						let min_right_value = subnode.min_right().value;
						subnode.delete(min_right_value);
						subnode.value = min_right_value;
					},
					_ => panic!("Something wrong!")
				}
			},
			&mut Some(ref mut subnode) if subnode.value != value => {
				subnode.delete(value);
			},
			_ => (),
		}

		if is_root {
			if let Some(ref new_self) = &mut tmp_self {
				*self = Node::new(new_self.value, new_self.left.clone(), new_self.right.clone())
			}
		}
	}
}
