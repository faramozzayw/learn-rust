mod tree;
mod node;

pub use tree::Tree;
pub(crate) use node::Node;

pub(crate) type Link<T> = Option<Box<Node<T>>>;