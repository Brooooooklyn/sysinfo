extern crate napi_build;

use std::env;

fn main() {
  let compile_target = env::var("TARGET").expect("TARGET");

  if compile_target == "aarch64-apple-darwin" {
    println!("cargo:rustc-link-lib=framework=IOKit");
  }

  napi_build::setup();
}
