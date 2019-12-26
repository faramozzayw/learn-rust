mod stack;

pub use stack::Stack;

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Link<T>,
}

#[allow(dead_code)]
impl<T> Node<T> 
{
    fn new(value: T, next: Link<T>) -> Self {
        Node { value, next }
    }
}