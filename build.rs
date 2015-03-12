// build.rs

// Bring in a dependency on an externally maintained `cc` package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    //gcc::compile_library("libmarisawrapper.a", &["src/marisa.cc"]);
    gcc::Config::new()
                .file("src/marisa.cc")
                .include("src")
                .compile("libmarisawrapper.a");
}
