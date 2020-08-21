mod mergesort;

fn main() {
  let mut xs: Vec<i32> = [4, 70, 6, 2, 3, 5, 8, 9, 1, 0].to_vec();
  //println!("{}, {:?}", xs.len(), xs);
  xs[1] = 7;
  println!("{}, {:?}", xs.len(), xs);
  mergesort::merge_sort(&mut xs);
  println!("{}, {:?}", xs.len(), xs);
}
