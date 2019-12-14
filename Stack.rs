use std::env;
use std::fs;
use std::io::stdin;
use std::fmt;
use std::mem;

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
struct Stack {
    head: Link,
}

#[allow(dead_code)]
impl Stack {
    fn new() -> Self {
        Stack { head: None }
    }
}

#[allow(dead_code)]
impl Stack {
    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            let value = node.value.clone();
            self.head = node.next;
            value
        })
    }

    fn peek(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn empty(&self) -> bool {
        match &self.head {
            None => true,
            _ => false,
        }
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut vector: Vec<i32> = Vec::new();
        let mut current_node = &self.head;

        while let Some(node) = &current_node {
            vector.push((&node.value).clone());
            current_node = &node.next;
        }

        vector
    }

   fn reverse(&mut self) {
        let mut vector: Vec<i32> = Vec::new();

        while let Some(node_value) = self.pop() {
            vector.push(node_value);
        }
        
        vector.reverse();
        
        while let Some(elem) = vector.pop() {
            self.push(elem);
        }
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

fn main() {
   let mut st = Stack::new();
   st.push(1);
   st.push(2);
   st.push(3);
   st.push(4);
   println!("Stack:\n{}", st);
}