//extern crate rand;

use rand::Rng;

static NUM_SUITS: i32 = 1;
static LOW_CARD: i32 = 0;
static HIGH_CARD: i32 = 3;

// Cards are 2..14
#[derive(Debug)]
struct Player {
  // We maintain the invariant that hand is never empty unless discard is as well, in which case
  // the players has won. If hand empties and discard is not empty, the contents of discard are
  // shuffled and moved to hand.
  hand: Vec<i32>, 
  discard: Vec<i32>, 
}

impl Player {
  pub fn new(cards: &[i32]) -> Player {
    return Player { hand: cards.to_vec(), discard: Vec::new() };
  }

  pub fn has_won(&self) -> bool {
    return self.hand.len() == 0;
  }

  pub fn next_card(&mut self) -> i32 {
    assert!(!self.has_won(), "");
    let card = self.hand.pop().unwrap();
    self.refresh_hand_maybe();
    return card;
  }

  pub fn refresh_hand_maybe(&mut self) {
    if self.hand.len() == 0 {
      self.hand.append(&mut self.discard);
    }
  }
}

#[derive(Debug)]
struct Game {
  a: Player,
  b: Player,
}

impl Game {
  pub fn new() -> Game {
    let d = random_deck();
    // It's random, so splitting it in half is as good as really dealing.
    let (a_cards, b_cards) = d.split_at(d.len()/2);
    return Game { a: Player::new(a_cards), b: Player::new(b_cards) };
  }

  pub fn play(&mut self) {
    let card = self.a.next_card();
    println!("card {}", card);
  }
}

fn deck() -> Vec<i32> {
  let mut v = Vec::new();
  for _ in 0..=NUM_SUITS {
    for c in LOW_CARD..=HIGH_CARD {
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
  //println!("{:?}", xs);
  //println!("{:?}", from);
  // print_type_of(xs);
  // print_type_of(from);
  for i in 0..xs.len() {
    xs[i] = remove_random(&mut from);
    //println!("{:?}", xs);
    //println!("{:?}", from);
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
  //println!("Remove {} at {}", x, i);
  return x;
}

pub fn demo() {
  let mut g = Game::new();
  println!("{:?}", g);
  g.play();
  println!("{:?}", g);
}
