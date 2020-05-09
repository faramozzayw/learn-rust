use super::{ Link, Node };
use std::fmt;
use std::{
	fmt::Display, 
	clone::Clone,
	marker::Copy,
};

#[derive(Debug)]
pub struct Stack<T> {
	head: Link<T>,
	len: usize,
}

#[allow(dead_code)]
impl<T> Stack<T>
where T: Display + Clone + Copy + ToString + PartialEq
{
	pub fn new() -> Self {
		Stack { head: None, len: 0 }
	}

	pub fn from_vec(mut v: Vec<T>) -> Self {
		let mut s = Stack::new();

		while let Some(elem) = v.pop() {
			s.push(elem);
		}

		s
	}
}

#[allow(dead_code)]
impl<T> Stack<T>
where T: Display + Clone + Copy + PartialEq
{
	pub fn push(&mut self, value: T) -> &mut Self {
		let new_node = Box::new(Node {
			value,
			next: self.head.take(),
		});

		self.head = Some(new_node);
		self.len += 1;

		self
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.len != 0 {
			self.len -= 1;
		}

		self.head.take().map(|node| {
			let value = node.value.clone();
			self.head = node.next;
			value
		})
	}

	pub fn peek(&self) -> Option<T> {
		self.head.as_ref().map(|node| node.value)
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| &mut node.value)
	}

	pub fn empty(&self) -> bool {
		match &self.head {
			None => true,
			_ => false,
		}
	}

	pub fn to_vec(&self) -> Vec<T> {
		let mut vector: Vec<T> = Vec::with_capacity(self.len);
		let mut current_node = &self.head;

		while let Some(node) = &current_node {
			vector.push((&node.value).clone());
			current_node = &node.next;
		}

		vector
	}

	pub fn reverse(&mut self) -> &mut Self {
		let mut vector: Vec<T> = Vec::with_capacity(self.len);

		while let Some(node_value) = self.pop() {
			vector.push(node_value);
		}

		vector.reverse();

		while let Some(elem) = vector.pop() {
			self.push(elem);
		}

		self
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn clear(&mut self) -> &mut Self {
		let mut current_node = self.head.take();

		while let Some(mut node) = current_node {
			current_node = node.next.take();
		}

		self.len = 0;

		self
	}

	pub fn get(&self, search_value: T) -> &Link<T> {
		let mut current_node = &self.head;

        while let Some(node) = &current_node {
			if node.value == search_value {
				return current_node;
			}

			current_node = &node.next;

		}
		
		&None
	}
}

impl<T> Display for Stack<T>
where T: ToString
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut fstr = String::default();
		let mut current_node = &self.head;

		while let Some(node) = &current_node {
			fstr += "|\t";
			fstr += &node.value.to_string();
			fstr += "\t|\n";
			current_node = &node.next;
		}

		write!(f, "{}", fstr)
	}
}

impl<T> Drop for Stack<T> {
	fn drop(&mut self) {
		let mut current_link = self.head.take();
		while let Some(mut node) = current_link {
			current_link = node.next.take();
		}
	}
}

impl<T> PartialEq for Stack<T>
where T: Display + Clone + Copy + PartialEq
{
	fn eq(&self, other: &Self) -> bool {
		let result: bool = if self.len() == other.len() {
			let a = self.to_vec();
			let b = other.to_vec();
			
			for (i, item) in a.iter().enumerate() {
				if item != &b[i] {
					return false
				} 
			}

			true
		} else {
			false
		};

		result
	}
}

