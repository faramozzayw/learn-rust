mod fzlib;
mod algo;

use fzlib::Tree;

fn main() {
    let mut tree: Tree<i32> = Default::default();

    tree.add(3);
    tree.add(-1);
    tree.add(4);

    println!("{:#?}", tree);
}