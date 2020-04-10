mod fzlib;
mod algo;

use fzlib::Tree;

fn main() {
let tree: Tree<i32> = Default::default();
    println!("{:#?}", tree);
}