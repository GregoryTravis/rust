// - Need to change to Vec
// https://stackoverflow.com/questions/41710952/allocate-array-onto-heap-with-size-known-at-runtime
fn main() {
  //let orig: [i32; 10] = [4, 7, 6, 2, 3, 5, 8, 9, 1, 0];
  //let xs = vec![0; orig.len()];
  let xs = [4, 7, 6, 2, 3, 5, 8, 9, 1, 0].to_vec();
  println!("hi");
  //println!("It has {} elements", orig.len());
  println!("It has {} elements", xs.len());
  println!("It has {:?} elements", xs);
  println!("It has {:?} elements", &xs[1..=3]);
}

/*
fn main() {
  let mut xs: [i32; 10] = [4, 7, 6, 2, 3, 5, 8, 9, 1, 0];
  println!("hi");

  let x: i32 = 5
  let y: i32 = 2;
  println!("n {} {} {}", x, y, y/x);

  println!("It has {} elements", xs.len());
  println!("It has {:?}", xs);

  merge_sort(&mut xs, 0, xs.len());

  println!("It has {:?}", xs);
}

// s inclusive, e exclusive
fn merge_sort(xs: &mut [i32], s: usize, e: usize) {
  if s == e-1 {
    // do nothing
  } else {
    mid = s + ((e - s) / 2);
    merge_sort(xs, s, mid);
    merge_sort(xs, mid, e)
  }
  println!("It has {} elements 2", xs.len());
}
*/
