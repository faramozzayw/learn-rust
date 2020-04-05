mod fzlib;
mod algo;

use fzlib::Tree;

fn main() {
    let tree: Tree<i32> = Tree::new(1);
    println!("{:#?}", tree);
}