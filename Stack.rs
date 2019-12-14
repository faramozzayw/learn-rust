use std::fmt;

extern crate rand;

use rand::Rng;

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

fn task(stack: &Stack) -> Stack {
  let mut vect = stack.to_vec();
  let len = vect.len() as i32;
  let mut sum: i32 = 0;
  
  for item in &vect {
    sum += item;
  }
  
  let average = sum / len;
  let mut new_stack = Stack::new();
  
  vect.reverse();
  
  for num in vect {
    if num > average {
      new_stack.push(num);
    }
  }
  
  new_stack
}

fn main() {
   let mut rng = rand::thread_rng();
   let mut st = Stack::new();
   
   for n in 1 ..= 9 {
     st.push(rng.gen_range(-50, 50));
   }
   
   println!("Stack:\n{}", st);
   
   let nst = task(&st);
   println!("New Stack:\n{}", nst);
}