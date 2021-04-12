use chrono::Local;
use env_logger;
use log::LevelFilter;
use std::io::Write;

#[allow(dead_code)]
pub fn main() -> Result<(), String> {
  // set up our logger prefix
  // i guess i'll move this somewhere more sensible soon

  env_logger::Builder::new()
    .format(|buf, record| {
      writeln!(
        buf,
        "{} [{}] - {}",
        Local::now().format("%Y-%m-%dT%H:%M:%S"),
        record.level(),
        record.args()
      )
    })
    .filter(None, LevelFilter::Info)
    .init();
  Ok(())
}
// tests below

#[cfg(test)]
mod tests {
  #[test]
  fn logger() {
    super::main().expect("logger panic");
    log::info!("Logger: {:?}", true);
    assert_eq!(2 + 2, 4);
  }
}
