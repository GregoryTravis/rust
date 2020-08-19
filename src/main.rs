fn main() {
  let xs = [4, 7, 6, 2, 3, 5, 8, 9, 1, 0].to_vec();
  println!("hi");
  println!("It has {} elements", xs.len());
  println!("It has {:?} elements", xs);
  println!("It has {:?} elements", &xs[1..=3]);
  merge_sort(&xs);
  //let sorted = merge_sort(&xs);
  //println!("It has {:?} elements", sorted);
}

fn merge_sort(arr: &[i32]) {
  if arr.len() == 1 {
    let mid = arr.len() / 2;
    let _sarr0: &[i32] = &arr[0..mid];
    let _sarr1: &[i32] = &arr[mid..arr.len()];
  } else if arr.len() > 1 {
  } else if arr.len() == 0 {
    panic!("??");
  }
}
