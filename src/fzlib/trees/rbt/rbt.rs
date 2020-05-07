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