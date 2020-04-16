use std::{
    fmt::Display, 
    clone::Clone,
	marker::Copy,
	fmt::Debug
};

use super::{ Link };

#[derive(Debug)]
pub(crate) struct Node<T> {
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

	fn min_node(&self) -> &Node<T> {
		match (&self.left, &self.right) {
			(Some(left), Some(right)) => {
				if left.value < self.value {
					return left.min_node();
				} else if right.value > self.value {
					return right.min_node();
				} 
			},
			(_, Some(right)) => {
				if right.value > self.value {
					return right.min_node();
				}
			},
			(Some(left), _) => {
				if left.value > self.value {
					return left.min_node();
				}
			},
			_ => (),
		};

		&self
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
where T: Debug + Display + Clone + Copy + PartialOrd
{
	pub(crate) fn insert(&mut self, value: T) {
		if self.value == value {
			panic!("The value '{}' is already exist in tree", value);
		}

		let node = if value < self.value {
			&mut self.left
		} else {
			&mut self.right
		};

		match node {
			&mut Some(ref mut subnode) => subnode.insert(value),
			&mut None => *node = Node::new_leaf(value),
		}
	}

	pub(crate) fn delete(&mut self, value: T) {
		if self.value == value {
			return ();
		}

		let node = if value < self.value {
			&mut self.left
		} else {
			&mut self.right
		}; 

		match node {
			&mut Some(ref subnode) if subnode.value == value && subnode.is_leaf() => {
				*node = None;
			},
			&mut Some(ref mut subnode) if subnode.value == value && !subnode.is_leaf() => {
				match subnode.children_count() {
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
	}
}