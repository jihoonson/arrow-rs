extern crate gcc;

use std::path::Path;

fn main() {
  let arrow_home = std::env::var("ARROW_HOME")
      .ok()
      .or(Some(String::from("/usr/local")))
      .expect("ARROW_HOME is not set.");

  let include_dir = format!("{}/include", arrow_home);
  let lib_dir = format!("{}/lib", arrow_home);
  let arrow_lib_name = "arrow";
  let arrow_io_lib_name = "arrow_io";
  let arrow_ipc_lib_name = "arrow_ipc";

  println!("cargo:rustc-link-search=native={}", lib_dir);
  println!("cargo:rustc-link-lib=dylib={}", arrow_lib_name);
  println!("cargo:rustc-link-lib=dylib={}", arrow_io_lib_name);
  println!("cargo:rustc-link-lib=dylib={}", arrow_ipc_lib_name);
  println!("cargo:include={}", include_dir);

  let cxx_flags = format!("-std=c++11 -I{} -L{} -l{} -l{} -l{}",
                          include_dir, lib_dir, arrow_lib_name, arrow_io_lib_name, arrow_ipc_lib_name);
  std::env::set_var("CXXFLAGS", cxx_flags);

  gcc::Config::new()
      .cpp(true)
      .file("src/ty.cc")
      .file("src/common/status.cc")
      .file("src/common/memory_pool.cc")
      .file("src/types/primitive.cc")
      .file("src/array.cc")
      .file("src/buffer.cc")
      .file("src/column.cc")
      .file("src/table.cc")
      .file("src/io/memory.cc")
      .file("src/ipc/adapter.cc")
      .compile("libtargetwrapper.a");
}
