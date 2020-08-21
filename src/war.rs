//extern crate rand;

use rand::Rng;

// Cards are 2..14
#[derive(Debug)]
struct Player {
  hand: Vec<i32>, 
  discard: Vec<i32>, 
}

#[derive(Debug)]
struct Game {
  a: Player,
  b: Player,
}

impl Game {
  pub fn new() -> Player {
    return Player { hand: Vec::new(), discard: Vec::new() };
  }
}

fn deck() -> Vec<i32> {
  let mut v = Vec::new();
  for _ in 0..=3 {
    for c in 2..=14 {
      v.push(c);
    }
  }
  return v;
}

fn random_deck() -> Vec<i32> {
  let mut d = deck();
  randomize(&mut d);
  return d;
}

fn randomize(xs: &mut [i32]) {
  let mut from = vec![0; xs.len()];
  from.clone_from_slice(xs);
  println!("{:?}", xs);
  println!("{:?}", from);
  // print_type_of(xs);
  // print_type_of(from);
  for i in 0..xs.len() {
    xs[i] = remove_random(&mut from);
    println!("{:?}", xs);
    println!("{:?}", from);
  }
  assert!(from.len() == 0, "uhoh");
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn remove_random(xs: &mut Vec<i32>) -> i32 {
//fn remove_random(xs: &mut [i32]) -> i32 {
  let mut rng = rand::thread_rng();
  let i = rng.gen_range(0, xs.len());
  let x = xs.remove(i);
  println!("Remove {} at {}", x, i);
  return x;
}

pub fn demo() {
  println!("{:?}", Game::new());
  let d = random_deck();
  println!("{:?}", d);
}
