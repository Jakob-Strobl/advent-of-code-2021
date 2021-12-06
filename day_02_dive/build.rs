use std::env;
use std::path::Path;
use std::fs;

// This isn't a great use of build.rs, but I just wanted to see if I could do it.
fn main() {
    // Get file to copy
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data = Path::new(&manifest_dir).join(Path::new("data/commands.txt"));
    println!("{:?}", data); // used for debugging. Find in target/build/<project things>/out/output

    // Get output path 
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{}", out_dir); // used for debugging
    let outfile = Path::new(&out_dir).join("commands.txt");

    fs::copy(data, outfile).unwrap();
}
