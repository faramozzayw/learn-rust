mod fzlib;
mod algo;

use fzlib::Tree;

fn main() {
    let mut tree: Tree<i32> = Tree::new(0);

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

	tree.delete(0);

	println!("{:#?}", tree);
}