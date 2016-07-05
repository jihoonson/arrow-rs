extern crate gcc;

use std::path::Path;

fn main() {

  let arrow_home = std::env::var("ARROW_HOME")
                    .ok()
                    .expect("ARROW_HOME is not set.");

  let include_dir = format!("{}/include", arrow_home);
  let lib_dir = format!("{}/lib", arrow_home);
  let lib_name = "arrow";

  println!("cargo:rustc-link-search=native={}", lib_dir);
  println!("cargo:rustc-link-lib=dylib={}", lib_name);
  println!("cargo:include={}", include_dir);

  let cxx_flags = format!("-std=c++11 -I{} -L{} -l{}",
    include_dir, lib_dir, lib_name);
  std::env::set_var("CXXFLAGS", cxx_flags);

  gcc::Config::new()
    .cpp(true)
    .file("src/type.cc")
    .compile("libtargetwrappers.a");
}