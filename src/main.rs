fn main() {
  let mut xs: Vec<i32> = [4, 70, 6, 2, 3, 5, 8, 9, 1, 0].to_vec();
  println!("{}, {:?}", xs.len(), xs);
  xs[1] = 7;
  println!("{}, {:?}", xs.len(), xs);
  merge_sort(&mut xs);
  println!("{}, {:?}", xs.len(), xs);
}

// pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
//     let len = self.len();

//fn merge_sort(xs: &mut Vec<i32>) {
fn merge_sort(xs: &mut [i32]) {
  if xs.len() > 1 {
    let mid = xs.len() / 2;
    let (sub0, sub1) = xs.split_at_mut(mid);
    merge_sort(sub0);
    merge_sort(sub1);
  } else if xs.len() == 1 {
    xs[0] = 111;
  } else {
    panic!("empty");
  }
}
