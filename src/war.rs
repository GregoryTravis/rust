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

pub fn demo() {
  println!("{:?}", Game::new());
}
