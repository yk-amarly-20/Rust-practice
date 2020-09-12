fn main() {
  let x = 5;

  let y = &x;

  let z = &y;

  dbg!(x);
  dbg!(y);
  dbg!(z);
}
