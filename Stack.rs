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
}

fn main() {
    let mut input = String::new();
    let mut st = Stack::new();
}