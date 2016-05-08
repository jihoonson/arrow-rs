extern crate gcc;

fn main() {

  let arrow_home = std::env::var("ARROW_HOME")
                    .ok()
                    .expect("ARROW_HOME is not set.");

  let cxx_flags = format!("-I{}/include -std=c++11", arrow_home);
  let ldlibs = format!("-l{}/lib", arrow_home);

  std::env::set_var("CXXFLAGS", cxx_flags);
  std::env::set_var("LDLIBS", ldlibs);
  gcc::Config::new()
    .cpp(true)
    .file("src/type.cc")
    .compile("libtype.a");
}
