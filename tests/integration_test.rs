use chiprs;

#[test]
fn logger() {
  chiprs::logger::main().expect("logger panic");
  log::info!("Logger: {:?}", true);
  assert_eq!(2 + 2, 4);
}

#[test]
fn chip8_games() {
  let games = chiprs::chip8::games();
  assert_eq!(games.len(), 9);
}
