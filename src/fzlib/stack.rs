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
    len: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack { head: None, len: 0 }
    }

    pub fn from_vec(mut v: Vec<i32>) -> Self {
        let mut s = Stack::new();

        while let Some(elem) = v.pop() {
            s.push(elem);
        }

        s
    }
}

#[allow(dead_code)]
impl Stack {
    pub fn push(&mut self, value: i32) -> &mut Self {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
        self.len += 1;

        self
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.len != 0 {
            self.len -= 1;
        }

        self.head.take().map(|node| {
            let value = node.value.clone();
            self.head = node.next;
            value
        })
    }

    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut i32> {
        self.head.as_mut().map(|node| &mut node.value)
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

    pub fn reverse(&mut self) -> &mut Self {
        let mut vector: Vec<i32> = Vec::with_capacity(self.len);

        while let Some(node_value) = self.pop() {
            vector.push(node_value);
        }

        vector.reverse();

        while let Some(elem) = vector.pop() {
            self.push(elem);
        }

        self
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) -> &mut Self {
        let mut current_node = self.head.take();

        while let Some(mut node) = current_node {
            current_node = node.next.take();
        }

        self.len = 0;

        self
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_basic_operation() {
        let mut s = Stack::new();

        s.push(1);
        s.push(2);

        assert_eq!(s.len(), 2);
        assert_eq!(s.pop(), Some(2));

        assert_eq!(s.len(), 1);

        s.push(3);
        s.push(4);
        s.push(5);

        assert_eq!(s.len(), 4);
        assert_eq!(s.empty(), false);

        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.len(), 3);

        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.len(), 2);

        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.len(), 1);

        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.len(), 0);

        assert_eq!(s.pop(), None);

        assert_eq!(s.empty(), true);


        let mut sv = Stack::from_vec(vec![1, 2, 3]);
        
        assert_eq!(sv.peek(), Some(1));
        
        sv.push(5);
        assert_eq!(sv.peek(), Some(5));

        assert_eq!(sv.pop(), Some(5));
        assert_eq!(sv.pop(), Some(1));
        assert_eq!(sv.pop(), Some(2));
    }

    #[test]
    fn it_peek() {
        let mut s = Stack::new();

        for i in 1..=10 {
            s.push(i);
            assert_eq!(s.peek(), Some(i));
        }

        s.clear();

        assert_eq!(s.peek(), None);
    }

    #[test]
    fn it_clear() {
        for i in 1..=30 {
            let mut s = Stack::new();

            for j in 1..=i {
                s.push(i * j);
            }

            assert_eq!(s.empty(), false);
            s.clear();

            assert_eq!(s.len(), 0);
            assert_eq!(s.peek(), None);
            assert_eq!(s.empty(), true);
        }
    }

    #[test]
    fn it_to_vec() {
        let a = Stack::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(a.to_vec(), vec![1, 2, 3, 4, 5]);

        let mut b = Stack::from_vec(vec![1, 2, 3]);
        b.push(4);
        b.push(5);
        assert_eq!(a.to_vec(), vec![1, 2, 3, 4, 5]);

        let mut c = Stack::new();
        c.push(1);
        c.push(2);
        c.push(3);
        assert_eq!(c.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn it_reverse() {
        let mut a = Stack::from_vec(vec![1, 2, 3, 4, 5]);
        let a_len = a.len();

        a.reverse();
        assert_eq!(a.len(), a_len);
        assert_eq!(a.to_vec(), vec![5, 4, 3, 2, 1]);

        let mut b = Stack::new();
        b.push(1);
        b.push(2);
        b.push(3);
        // b: [3, 2, 1]
        let b_len = b.len();
        
        b.reverse();
        assert_eq!(b.len(), b_len);
        assert_eq!(b.to_vec(), vec![1, 2, 3]);

        let mut c = Stack::from_vec(vec![1, 2, 3, 4, 5]);

        c.pop();
        c.pop();
        c.push(0);

        let c_len = c.len();

        c.reverse();
        assert_eq!(c.len(), c_len);
        assert_eq!(c.to_vec(), vec![5, 4, 3, 0]);
    }
}
