#[allow(unused_imports)]
use super::{
	Tree
};

#[cfg(test)]
mod tree_tests {
	use super::Tree;

	#[test]
	fn it_is_empty() {
		// Default value for `i32` is `0`
		let mut tree: Tree<i32> = Default::default();
		tree.delete(0);
		assert!(tree.is_empty());

		let mut tree = Tree::new(5);
		tree.delete(5);
		assert!(tree.is_empty());

		let mut tree = Tree::new(5);

		tree
			.add(2)
			.add(12)
			.add(-4)
			.add(3)
			.add(9)
			.add(21)
			.add(19)
			.add(25);
		
		tree.delete(5);
		assert_eq!(tree.is_empty(), false);
	}

	#[test]
	fn it_add() {
		let mut tree = Tree::new(5);
		tree.delete(5);
		assert!(tree.is_empty());
		
		tree.add(3);
		assert_eq!(tree.is_empty(), false);
		assert_eq!(tree.root.unwrap().value, 3);
	}
}