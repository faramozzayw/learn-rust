extern crate rand;

use rand::Rng;
use std::collections::LinkedList;

fn main() {
    let mut rng = rand::thread_rng();

    let mut queue = LinkedList::new();
    let mut result = LinkedList::new();

    for _ in 1..=10 {
        queue.push_back(rng.gen_range(-50, 50));
    }

    println!("Queue:\n{:?}\n", queue);

    // (value, index)
    let mut min = (queue.front().unwrap(), 0);
    let mut max = (queue.front().unwrap(), 0);

    for (i, elem) in queue.iter().enumerate() {
        match elem {
            _k if elem > max.0 => {
                max.0 = elem;
                max.1 = i;
            }
            _k if elem < min.0 => {
                min.0 = elem;
                min.1 = i;
            }
            _ => (),
        }
    }

    println!("max: {:?}", max);
    println!("min: {:?}", min);

    if min.1 < max.1 {
        for (i, elem) in queue.iter().enumerate() {
            match i {
                _ if i <= min.1 || max.1 <= i => {
                    result.push_back(elem);
                }
                _ => (),
            }
        }
    } else {
        for (i, elem) in queue.iter().enumerate() {
            match i {
                _ if i >= min.1 || max.1 >= i => {
                    result.push_back(elem);
                }
                _ => (),
            }
        }
    }

    println!("Result queue: {:?}\n", result);
}