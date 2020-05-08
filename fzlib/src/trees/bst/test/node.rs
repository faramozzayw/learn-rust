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
	#[should_panic]
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

				assert_eq!(node.min_node().value, -5);
			},
			_ => (),
		}

		let mut node = Node::new_leaf(-5);
		
		match &mut node {
			Some(node) => {
				node.insert(1);
				node.insert(32);
				node.insert(-1);
				node.insert(-4);

				assert_eq!(node.min_node().value, -5);
			},
			_ => (),
		}
	}

	#[test]
	fn it_delete() {
		let mut node = Node::new_leaf(5);

		match &mut node {
			Some(node) => {
				node.insert(1);
				node.insert(32);

				node.delete(1);

				assert_eq!(node.value, 5);
				
				match &mut node.right {
					Some(node) => assert_eq!(node.value, 32),
					_ => (),
				}
			},
			_ => panic!("Something bad wrong..."),
		}

		let mut node = Node::new_leaf(25);

		match &mut node {
			Some(node) => {
				node.insert(5);
				node.insert(1);

				node.delete(5);

				match &mut node.left {
					Some(node) => assert_eq!(node.value, 1),
					_ => (),
				}

				node.delete(1);

				match &mut node.left {
					None => assert!(true),
					Some(_) => assert!(false),
				}
			},
			_ => panic!("Something bad wrong..."),
		}
	}

	#[test]
	fn it_delete_nothing() {
		let mut node = Node::new_leaf(5);
		
		match &mut node {
			Some(node) => {
				for key in -4..=4 {
					node.delete(key);
				}

				for key in 6..=10 {
					node.delete(key);
				}
			},
			_ => (),
		}
	}

	#[test]
	fn it_delete_root_isnt_leaf() {
		
		let mut node = Node::new_leaf(3);
		match &mut node {
			Some(node) => {
				node.insert(2);
				node.insert(4);
				node.delete(3);
				
				assert_eq!(node.value, 4);

				match &mut node.left {
					Some(ref mut subnode) => {
						assert_eq!(subnode.value, 2);
					},
					None => assert!(false),
				}

				assert_eq!(node.value, 4);
				assert!(!node.is_leaf());

				node.delete(2);
				assert!(node.is_leaf());

				node.delete(4);
			},
			_ => panic!("Something bad wrong..."),
		}
	}
}
