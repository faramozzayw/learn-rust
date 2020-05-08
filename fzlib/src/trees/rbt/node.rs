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
}

#[allow(dead_code, unused_variables)]
impl<T> Node<T> {
	pub(crate) fn insert(&mut self, value: T) {
		unimplemented!();
	}

	pub(crate) fn delete(&mut self, value: T) {
		unimplemented!();
	}
}