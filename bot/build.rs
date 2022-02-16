use std::process::Command;
fn main() {
  let output = Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
  let git_hash = String::from_utf8(output.stdout).unwrap();
  println!("cargo:rustc-env=GIT_HASH={}", git_hash);

  let rustc_output = Command::new("rustc").arg("-V").output().unwrap();
  let rust_data = String::from_utf8(rustc_output.stdout).unwrap();
  println!("cargo:rustc-env=RUST_DATA={}", rust_data);
}
