use fzlib::prelude::*;

fn main() {
	let mut stack: Stack<i32> = Stack::new();
	println!("{:#?}", &stack);


	for i in 0..5 {
		stack.push(i);
	}
	println!("{:#?}", &stack);

	println!("getting 2: \n{:#?}", stack.get(2));

	for _ in 0..3 {
		if let Some(elem) = stack.pop() {
			println!("Popped elem: {:?}", elem);
		}
	}

	println!("{:#?}", &stack);

}