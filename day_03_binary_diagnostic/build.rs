use std::env;
use std::path::Path;
use std::fs;

const FILE_NAME: &str = "binary.txt";

// This isn't a great use of build.rs, but I just wanted to see if I could do it.
fn main() {
    // Get file to copy
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let data = Path::new(&manifest_dir).join(Path::new(&format!("data/{}", FILE_NAME)));

    // Get output path 
    let out_dir = env::var("OUT_DIR").unwrap();
    let outfile = Path::new(&out_dir).join(FILE_NAME);

    fs::copy(data, outfile).unwrap();
}
