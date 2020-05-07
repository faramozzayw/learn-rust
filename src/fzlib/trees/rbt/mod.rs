mod rbt;
mod node;
mod color;
mod test;

pub use rbt::RBT;

pub(crate) use node::Node;
pub(crate) use color::Color;

pub(crate) type Link<T> = Option<Box<Node<T>>>;