//extern crate rand;

//use rand::Rng;

mod fz {
    use std::fmt;

    type Link = Option<Box<Node>>;

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Link,
    }

    #[allow(dead_code)]
    impl Node {
        fn new(value: i32, next: Link) -> Self {
            Node { value, next }
        }
    }

    #[derive(Debug)]
    pub struct Stack {
        head: Link,
        len: usize
    }

    impl Stack {
        pub fn new() -> Self {
            Stack { head: None, len: 0 }
        }
    }

    #[allow(dead_code)]
    impl Stack {
        pub fn push(&mut self, value: i32) {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });

            self.head = Some(new_node);
            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<i32> {
            self.len -= 1;
            self.head.take().map(|node| {
                let value = node.value.clone();
                self.head = node.next;
                value
            })
        }

        pub fn peek(&self) -> Option<&i32> {
            self.head.as_ref().map(|node| &node.value)
        }

        pub fn empty(&self) -> bool {
            match &self.head {
                None => true,
                _ => false,
            }
        }

        pub fn to_vec(&self) -> Vec<i32> {
            let mut vector: Vec<i32> = Vec::with_capacity(self.len);
            let mut current_node = &self.head;

            while let Some(node) = &current_node {
                vector.push((&node.value).clone());
                current_node = &node.next;
            }

            vector
        }

        pub fn reverse(&mut self) {
            let mut vector: Vec<i32> = Vec::with_capacity(self.len);

            while let Some(node_value) = self.pop() {
                vector.push(node_value);
            }

            vector.reverse();

            while let Some(elem) = vector.pop() {
                self.push(elem);
            }
        }
        
        pub fn len(&self) -> usize {
            self.len
        }
    }

    impl fmt::Display for Stack {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut fstr = String::default();
            let mut current_node = &self.head;

            while let Some(node) = &current_node {
                fstr += "|\t";
                fstr += &node.value.to_string();
                fstr += "\t|\n";
                current_node = &node.next;
            }

            write!(f, "{}", fstr)
        }
    }

    impl Drop for Stack {
        fn drop(&mut self) {
            let mut current_link = self.head.take();
            while let Some(mut node) = current_link {
                current_link = node.next.take();
            }
        }
    }
}

use crate::fz::Stack;

fn main() {
    //let mut rng = rand::thread_rng();
    let mut st = Stack::new();

    for n in 1..=9 {
        st.push(n);
    }

    println!("Stack:\n{}", st);
}