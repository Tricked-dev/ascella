use std::process::Command;
fn main() -> Result<(), Box<dyn std::error::Error>> {
  let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
  let git_hash = String::from_utf8(output.stdout).unwrap();
  println!("cargo:rustc-env=GIT_HASH={}", git_hash);

  let rustc_output = Command::new("rustc").arg("-V").output().unwrap();
  let rust_data = String::from_utf8(rustc_output.stdout).unwrap();
  println!("cargo:rustc-env=RUST_DATA={}", rust_data);

  rosetta_build::config()
    .source("en", "../locales/en.json")
    .source("pl", "../locales/pl.json")
    .source("es", "../locales/es.json")
    .source("fr", "../locales/fr.json")
    .source("nl", "../locales/nl.json")
    .source("bg", "../locales/bg.json")
    .fallback("en")
    .generate()?;

  Ok(())
}
