mod stack;
mod trees;
//mod tests;

pub use stack::Stack;
pub use trees::Tree;

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}