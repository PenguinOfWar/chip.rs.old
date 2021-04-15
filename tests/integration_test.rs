use chiprs;

#[test]
fn chip_games() {
  let games = chiprs::games();
  assert_eq!(games.len(), 9);
}
