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
	#[test]
	#[should_panic(expected = "assertion failed: `(left == right)`")]
	fn it_is_leaf_panic() {
		let node = Node::new_leaf(1);
		assert_eq!(node.unwrap().is_leaf(), false);

		let node = Node::new(1, None, Node::new_leaf(2));
		assert!(node.is_leaf(), true);

		let node = Node::new(1, Node::new_leaf(23), Node::new_leaf(2));
		assert!(node.is_leaf(), true);
	}


	#[test]
	fn it_insert() {
		let mut node = Node::new_leaf(5);
		match &mut node {
			Some(node) => {
				node.insert(1);
				node.insert(6);
				
				assert_eq!(node.left.as_ref().unwrap().value, 1);
				assert_eq!(node.right.as_ref().unwrap().value, 6);

				node.insert(15);
				match &mut node.right {
					Some(node) => {
						assert_eq!(node.right.as_ref().unwrap().value, 15);

						node.insert(3);
						assert_eq!(node.left.as_ref().unwrap().value, 3);
					},
					_ => (),
				}
			},
			_ => (),
		}
	}

	#[test]
	#[should_panic(expected = "The value \'4\' is already exist in tree")]
	fn it_insert_4_panic() {
		let node = Node::new_leaf(4);
		node.unwrap().insert(4);
	}

	#[test]
	fn it_min_node() {
		let mut node = Node::new_leaf(4);
		
		match &mut node {
			Some(node) => {
				node.insert(1);
				node.insert(6);
				node.insert(61);
				node.insert(-5);

				assert_eq!(node.min_node(), -5);
			},
			_ => (),
		}
	}
}
