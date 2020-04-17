mod tree;
mod node;
mod test;

pub use tree::Tree;
pub(crate) use node::Node;

pub(crate) trait TreeNodeMark: NodeClone {}

pub(crate) trait NodeClone {
    fn clone_box(&self) -> Box<dyn TreeNodeMark>;
}

impl<T> NodeClone for T
where T: 'static + TreeNodeMark + Clone
{
    fn clone_box(&self) -> Box<dyn TreeNodeMark> {
		Box::new(self.clone())
	}
}

impl Clone for Box<dyn TreeNodeMark>
{
	fn clone(&self) -> Box<dyn TreeNodeMark> {
		self.clone_box()
	}
}

pub(crate) type Link<T> = Option<Box<Node<T>>>;