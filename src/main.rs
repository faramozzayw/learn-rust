mod fzlib;
mod algo;

use fzlib::*;

fn main() {
	let mut rbt: RBT<i32> = RBT::new(5);
	
	println!("{:#?}", rbt);
}