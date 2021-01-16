// Todo
// - generic Num
// - unify impls -- pass logic in
// - check IS and CS
// - CS class?
// - test
// - normalize (maybe not necessary)
// - implement iter for IntervalSet
// - generics for TC tag
// - better cs merge

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
  let cs0 = is_to_changes(&is0);
  println!("{:?}", cs0);
  let cs1 = is_to_changes(&is1);
  println!("{:?}", cs1);
  let merged = merge_changesets(&cs0, &cs1);
  println!("{:?}", merged);
  //println!("{:?}", union(is0, is1));
  // println!("{:?}", intersection(is0, is1));
  // println!("{:?}", difference(is0, is1));
  // println!("{:?}", difference(is1, is0));
}

#[derive(Copy, Clone, Debug)]
enum OC {
  Open,
  Close,
}

#[derive(Copy, Clone, Debug)]
struct Change {
  i: i32,
  oc: OC,
}

#[derive(Debug)]
struct TaggedChange {
  c: Change,
  tag: i32,
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

fn merge_changesets(cs0: &Vec<Change>, cs1: &Vec<Change>) -> Vec<TaggedChange> {
  let mut tcs = Vec::new();
  let mut i0 = 0;
  let mut i1 = 0;
  while i0 < cs0.len() && i1 < cs1.len() {
    if i0 == cs0.len() {
      tcs.push(TaggedChange { c: cs1[i1], tag: 1 });
      i1 += 1;
    } else if i1 == cs1.len() {
      tcs.push(TaggedChange { c: cs0[i0], tag: 0 });
      i0 += 1;
    } else {
      if cs0[i0].i < cs1[i1].i {
        tcs.push(TaggedChange { c: cs0[i0], tag: 0 });
        i0 += 1;
      } else {
        tcs.push(TaggedChange { c: cs1[i1], tag: 1 });
        i1 += 1;
      }
    }
  }
  return tcs;
}

//fn union(
