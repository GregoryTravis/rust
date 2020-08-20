fn main() {
  let mut xs: Vec<i32> = [4, 70, 6, 2, 3, 5, 8, 9, 1, 0].to_vec();
  //println!("{}, {:?}", xs.len(), xs);
  xs[1] = 7;
  println!("{}, {:?}", xs.len(), xs);
  merge_sort(&mut xs);
  println!("{}, {:?}", xs.len(), xs);
}

// pub fn split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
//     let len = self.len();

//fn merge_sort(xs: &mut Vec<i32>) {
fn merge_sort(xs: &mut [i32]) {
  //println!("merge_sort {}, {:?}", xs.len(), xs);
  if xs.len() > 1 {
    let mid = xs.len() / 2;
    let (sub0, sub1) = xs.split_at_mut(mid);
    merge_sort(sub0);
    merge_sort(sub1);
    let v0 = sub0.to_vec();
    let v1 = sub1.to_vec();
    merge(xs, v0, v1);
  } else if xs.len() == 1 {
    // xs[0] = 111;
  } else {
    panic!("empty");
  }
}

fn merge(xs: &mut [i32], v0: Vec<i32>, v1: Vec<i32>) {
  //println!("merge v0 {}, {:?}", v0.len(), v0);
  //println!("merge v1 {}, {:?}", v1.len(), v1);
  let mut i = 0;
  let mut i0 = 0;
  let mut i1 = 0;
  while i0 < v0.len() && i1 < v1.len() {
    if v0[i0] <= v1[i1] {
      xs[i] = v0[i0];
      i0 += 1;
    } else {
      xs[i] = v1[i1];
      i1 += 1;
    }
    i += 1;
  }
  while i0 < v0.len() {
    xs[i] = v0[i0];
    i += 1;
    i0 += 1;
  }
  while i1 < v1.len() {
    xs[i] = v1[i1];
    i += 1;
    i1 += 1;
  }
  //println!("merge xs {}, {:?}", xs.len(), xs);
  //println!("{} {} {} {}", i0, v0.len(), i1, v1.len());
  assert!(i0 == v0.len() && i1 == v1.len(), "uhoh");
}
