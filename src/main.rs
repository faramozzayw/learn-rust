mod fzlib;
mod algo;

use fzlib::Tree;


fn main() {
    //let mut tree: Tree<f32> = Default::default();
    let mut tree: Tree<i32> = Tree::new(5);

	/*
	tree
		.add(3.0)
    	//.add(-1)
		.add(4.0)
		//.add(-5)
		.add(2.0)
		.add(1.5)
		.add(1.75)
		.add(0.75)
		.add(0.5)
		.add(1.0);
	*/

	tree
		.add(2)
		.add(12)
		.add(-4)
		.add(3)
		.add(9)
		.add(21)
		.add(19)
		.add(25);

	println!("{:#?}", tree);

	//tree.delete(-1);
	tree.delete(12);
	//tree.delete(19);

	//tree.delete(0);

    println!("{:#?}", tree);
	
}