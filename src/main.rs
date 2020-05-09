use fzlib::prelude::*;

fn main() {
	// let mut rbt: RBT<i32> = RBT::new(5);
	// println!("{:#?}", rbt);

	let mut v = vec![1, 5, -3, 2];
	println!("{:?}", v);
	selection_sort(&mut v);
	println!("{:?}", v);
}