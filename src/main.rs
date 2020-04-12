mod fzlib;
mod algo;

use fzlib::Tree;


fn main() {
    let mut tree: Tree<i32> = Default::default();

	tree
		.add(3)
    	.add(-1)
		.add(4)
		.add(-5)
		.add(1);

    println!("{:#?}", tree);
}