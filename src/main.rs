mod fzlib;

use fzlib::Stack;

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