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

use super::{ Link, Node, Color };

#[allow(dead_code)]
#[derive(Debug)]
pub struct RBT<T> {
    root: Link<T>
}

pub trait Tree<T> {
	fn add(&mut self, _: T) -> Result<&mut Self, ()>;
	fn delete(&mut self, _: T) -> Result<&mut Self, ()>;
	fn is_empty(&self) -> bool;
}

#[allow(dead_code)]
impl<T> RBT<T> 
where T: Display + Debug + Clone + Copy + Default
{
    pub fn new(value: T) -> Self {
        let root = Node::new(value, None, None, Color::Black);
        let root = Box::new(root);
        let root = Some(root);

        Self { root }
    }
}

impl<T> Default for RBT<T>
where T: Default + Debug + Display  + Clone + Copy
{
	fn default() -> Self {
		let root: T = Default::default(); 
		let root = Node::new_link(root, None, None, Color::Black);
		
		RBT { root }
	}
}

impl<T> Tree<T> for RBT<T>
where T: Debug + Display + Clone + Copy + PartialOrd + Ord + Default
{
	fn is_empty(&self) -> bool {
		self.root.is_none()
	}

	fn add(&mut self, value: T) -> Result<&mut Self, ()> {
		#[allow(unused_must_use)]
		match &mut self.root {
			// TODO: refactoring
			Some(node) => {
				node.insert(value);
				()
			},
			None => self.root = Node::new_link(value, None, None, Color::Black),
		}

		Ok(self)
	}

	fn delete(&mut self, value: T) -> Result<&mut Self, ()> {
		unimplemented!();
	}

}