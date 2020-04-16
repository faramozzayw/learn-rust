#[allow(unused_imports)]
use super::{
	Node
};

#[cfg(test)]
mod node_test {
	use super::Node;
	
	#[test]
	fn it_new() {
		let node = Node::new(1, None, None);

		assert_eq!(node.value, 1);
		assert_eq!(node.left.is_none(), true);
		assert_eq!(node.right.is_none(), true);

		let node = Node::new(2, Some(Box::new(node)), None);
		
		assert_eq!(node.value, 2);
		assert_eq!(node.left.is_none(), false);
		assert_eq!(node.left.is_some(), true);

		assert_eq!(node.right.is_none(), true);
		assert_eq!(node.right.is_some(), false);


		let node = Node::new(
			2,
			Some(Box::new(node)),
			Some(Box::new(Node::new(1, None, None)))
		);
		
		assert_eq!(node.value, 2);
		assert_eq!(node.left.is_none(), false);
		assert_eq!(node.left.is_some(), true);

		assert_eq!(node.right.is_none(), false);
		assert_eq!(node.right.is_some(), true);

	}

	#[test]
	fn it_children_count() {
		let node = Node::new_leaf(1);

		assert_eq!(node.unwrap().children_count(), 0);

		let node = Node::new_leaf(-1254.123);
		assert_eq!(node.unwrap().children_count(), 0);
		
		let node = Node::new(5, Node::new_leaf(3), None);
		assert_eq!(node.children_count(), 1);

		let node = Node::new(true, Node::new_leaf(false), None);
		assert_eq!(node.children_count(), 1);

		let node = Node::new(2 as usize, Node::new_leaf(4 as usize), None);
		assert_eq!(node.children_count(), 1);

		let node = Node::new(5, Node::new_leaf(3), Node::new_leaf(-3));
		assert_eq!(node.children_count(), 2);
	}

	#[test]
	fn it_is_leaf() {
		let node = Node::new_leaf(1);
		assert_eq!(node.unwrap().is_leaf(), true);

		let node = Node::new(1, None, Node::new_leaf(2));
		assert_eq!(node.is_leaf(), false);

		let node = Node::new(1, Node::new_leaf(23), Node::new_leaf(2));
		assert_eq!(node.is_leaf(), false);
	}
}