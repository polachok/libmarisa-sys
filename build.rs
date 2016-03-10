// build.rs

// Bring in a dependency on an externally maintained `cc` package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/marisa.cc")
                .include("src")
                .cpp(true)
                .compile("libmarisawrapper.a");
    println!("cargo:rustc-flags=-l marisa");
}
