extern crate rand;

use std::collections::LinkedList;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut queue = LinkedList::new();
    let mut tmp = LinkedList::new();
    let mut result = LinkedList::new();

    for _ in 1 ..= 15 {
        queue.push_back(rng.gen_range(-50, 50));
    }

    println!("Queue: {:?}\n", queue);

    let first = queue.pop_front();
    let mut last = 0;

    let mut i: usize = 0;
    let size = queue.len() - 1;

    while let Some(elem) = queue.pop_front() {
        match i {
            _ if i < size => {
                tmp.push_back(elem);
            }
            _ if i == size => {
                last = elem;
                if let Some(x) = first {
                    tmp.push_back(x);
                }
            }
            _ => break,
        }

        i += 1;
    }
    
    result.push_back(last);
    
    while let Some(elem) = tmp.pop_front() {
        result.push_back(elem);
    }

    println!("Result queue:\n{:?}\n", result);
}