mod fzlib;
mod algo;

//use fzlib::Stack;
use algo::*;

fn main() {
    let arr: Vec<i32> = vec![1, 2, 15, 4, 5, 6];
    let res = find_max(&arr);
    println!("Max elem of arr {:?} is {:?}.\n", &arr, res);

    let arr: Vec<f32> = vec![1.2, 2.01, 15.12, 40.23, 5.0, 6.0];
    let res = find_max(&arr);
    println!("Max elem of arr {:?} is {:?}.\n", &arr, res);

    let arr: Vec<f64> = vec![100.2, 56.5, 75.0, 112.15, 666.0];
    let res = find_max(&arr);
    println!("Max elem of arr {:?} is {:?}.\n", &arr, res);
}

/*
fn main() {
  println!("Hello, world!");
  
  let mut st = Stack::new();

  for n in 1..=9 {
      st.push(n);
  }

  println!("Stack:\n{}", st);
  st.clear();
  println!("Stack:\n{}", st);
  println!("Empty? {}", st.empty());
  println!("Length: {}", st.len());

  for n in 1..=5 {
    st.push(n);
  }

  println!("Stack:\n{}", st);
    if let Some(x) = st.peek_mut() {
      println!("{}", x);
      *x = 666;
  }

  println!("Stack:\n{}", st);

  let mut vst = Stack::from_vec(vec![1, 2, 3]);
  vst
    .push(15)
    .push(4);

  println!("vst:\n{}", vst);
  println!("vst:\n{:?}", vst.to_vec());
}
*/