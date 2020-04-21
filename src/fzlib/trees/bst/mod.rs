mod tree;
mod node;
mod test;
mod tree_node_mark;

pub use tree::Tree;

pub(crate) use node::Node;
pub(crate) use tree_node_mark::TreeNodeMark;

pub(crate) type Link<T> = Option<Box<Node<T>>>;