// build.rs
// sparkle-sys

fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=framework={}", dir);
}
