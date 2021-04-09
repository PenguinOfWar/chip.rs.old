// define our main mod function
// i might be doing something wrong but all the code samples for 
// lib style exports seem to suggest allowing dead code  ¯\_(ツ)_/¯
#[allow(dead_code)]
pub fn main() {
  println!("called `chip::main()`");
}

// couldnt work out how to 'export' a list of our games so...
// function to the rescue
#[allow(dead_code)]
pub fn games() -> Vec<&'static str> {
  let games: Vec<&str> = vec![
    "Invaders", "Brix", "Tetris", "Pong", "UFO", "IBM", "Missile", "Tank", "Maze",
  ];

  return games;
}