use chiprs;

#[test]
fn chip8_games() {
  let games = chiprs::chip8::games();
  assert_eq!(games.len(), 9);
}
