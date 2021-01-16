// Todo
// - generic Num
// - unify impls -- pass logic in
// - check IS and CS
// - CS class?
// - test
// - normalize (maybe not necessary)
// - implement iter for IntervalSet

#[derive(Debug)]
struct IntervalSet {
  intervals: Vec<(i32, i32)>,
}

pub fn demo() {
  let is0 = IntervalSet {
    intervals: [(1, 4), (5, 8), (12, 20)].to_vec(),
  };
  let is1 = IntervalSet {
    intervals: [(3, 6), (7, 11), (18, 24)].to_vec(),
  };
  println!("{:?}", is0);
  println!("{:?}", is1);
  println!("{:?}", is_to_changes(&is0));
  //println!("{:?}", union(is0, is1));
  // println!("{:?}", intersection(is0, is1));
  // println!("{:?}", difference(is0, is1));
  // println!("{:?}", difference(is1, is0));
}

#[derive(Debug)]
enum OC {
  Open,
  Close,
}

#[derive(Debug)]
struct Change {
  i: i32,
  oc: OC,
}

fn is_to_changes(iss: &IntervalSet) -> Vec<Change> {
  let mut changes = Vec::new();
  // let c = Change { i: 3, oc: OC::Open };
  for (s, e) in iss.intervals.iter() {
    changes.push(Change { i: *s, oc: OC::Open });
    changes.push(Change { i: *e, oc: OC::Close });
  }
  return changes;
}

//fn union(
