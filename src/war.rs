//extern crate rand;

use std::time::Instant;

use rand::Rng;

static NUM_GAMES: i32 = 10000;

// static NUM_SUITS: i32 = 1;
// static LOW_CARD: i32 = 0;
// static HIGH_CARD: i32 = 3;
static NUM_SUITS: i32 = 4;
static LOW_CARD: i32 = 2;
static HIGH_CARD: i32 = 14;

// Cards are 2..14
#[derive(Debug)]
struct Player {
  // We maintain the invariant that hand is never empty unless discard is as well, in which case
  // the players has lost. If hand empties and discard is not empty, the contents of discard are
  // shuffled and moved to hand.
  hand: Vec<i32>, 
  discard: Vec<i32>, 
}

impl Player {
  pub fn new(cards: &[i32]) -> Player {
    return Player { hand: cards.to_vec(), discard: Vec::new() };
  }

  pub fn has_lost(&self) -> bool {
    return self.hand.len() == 0;
  }

  pub fn next_card(&mut self) -> i32 {
    assert!(!self.has_lost(), "");
    let card = self.hand.pop().unwrap();
    self.refresh_hand_maybe();
    return card;
  }

  pub fn refresh_hand_maybe(&mut self) {
    if self.hand.len() == 0 {
      self.hand.append(&mut self.discard);
    }
  }

  pub fn give(&mut self, card: i32) {
    self.discard.push(card);
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

  // returns:
  //   - 0 for a winning, 1 for b
  //   - # of rounds
  //   - # of wars
  pub fn play(&mut self) -> (usize, i32, i32) {
    let mut num_rounds: i32 = 0;
    let mut num_wars: i32 = 0;

    let mut a_cards: Vec<i32> = Vec::new();
    let mut b_cards: Vec<i32> = Vec::new();

    // The number of cards the players must provide before we compare them.  On a normal turn, the
    // value is 1 -- each player must supply a card, and then we compare them and the winner wins
    // the hand. If there is a tie, war has broken out, and each player must supply four cards
    // before we compare again. If a player is out of cards at any time, they have lost.
    // Cards are played onto stacks, one for each player; when it is time to compare, we compare
    // the last values on the two stacks.
    let mut ante: i32 = 1;
    while !self.a.has_lost() && !self.b.has_lost() {
      if ante > 0 {
        // Players must play some cards
        let a_card = self.a.next_card();
        let b_card = self.b.next_card();
        a_cards.push(a_card);
        b_cards.push(b_card);
        ante -= 1;
      } else {
        // Ready to compare cards.
        //assert!(!a_cards.is_empty(), "");
        //assert!(!b_cards.is_empty(), "");
        let top_a_card: i32 = *(a_cards.last().unwrap());
        let top_b_card: i32 = *(b_cards.last().unwrap());
        // if top_a_card == top_b_card {
        //   panic!("this should happen");
        // }
        //println!("WHA {} {} {} {} {}", top_a_card, top_b_card, (top_a_card<=top_b_card), (top_a_card>=top_b_card), (top_a_card==top_b_card));
        if top_a_card < top_b_card {
          // B has won, and gets all the cards in the stacks
          for card in a_cards.iter() {
            self.b.give(*card);
          }
          for card in b_cards.iter() {
            self.b.give(*card);
          }
          a_cards.clear();
          b_cards.clear();
          ante = 1;
        } else if top_a_card > top_b_card {
          // A has won, and gets all the cards in the stacks
          for card in a_cards.iter() {
            self.a.give(*card);
          }
          for card in b_cards.iter() {
            self.a.give(*card);
          }
          a_cards.clear();
          b_cards.clear();
          ante = 1;
        } else {
          // War has broken out! Up the ante and loop
          ante += 4;
          num_wars += 1;
          //panic!("this should happen 2");
        }
      }
      num_rounds += 1;
    }

      //let a_card = self.a.next_card();
      //let b_card = self.b.next_card();
      ////println!("draw {} {}", a_card, b_card);
      //if a_card <= b_card {
      //  self.b.give(a_card);
      //  self.b.give(b_card);
      //} else {
      //  self.a.give(a_card);
      //  self.a.give(b_card);
      //}
      ////num_rounds += 1;
      ////println!("{:?}", self);
    //}

    assert!(self.a.has_lost() != self.b.has_lost(), "");

    //let which: &str = if self.a.has_won() { "a" } else { "b" };
    //println!("Win by {} after {} rounds", which, num_rounds);
    let return_code = if self.a.has_lost() { 1 } else { 0 };
    return (return_code, num_rounds, num_wars);
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
  let mut scores = vec![0; 2];
  let mut total_num_rounds = 0;
  let mut total_num_wars = 0;
  let start = Instant::now();
  for _ in 0..NUM_GAMES {
    let mut g = Game::new();
    //println!("{:?}", g);
    let (winner_i, num_rounds, num_wars) = g.play();
    scores[winner_i] += 1;
    total_num_rounds += num_rounds;
    total_num_wars += num_wars;
    //println!("{} wins {} rounds {} wars", winner_i, num_rounds, num_wars);
  }
  let duration: std::time::Duration = start.elapsed();
  println!("Elapsed time: {:.2?}", duration);
  println!("Elapsed time: {:.2?}s", duration.as_secs_f64());
  println!("gps {}", NUM_GAMES as f64 / duration.as_secs_f64());
  println!("scores: {:?}", scores);
  println!("total {} rounds {} wars", total_num_rounds, total_num_wars);
}
