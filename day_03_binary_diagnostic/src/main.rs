use std::collections::btree_set::Union;
use std::io::BufRead;
use std::ops::Deref;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

struct Unit(u8);

impl Deref for Unit {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        return &self.0
    }
}

impl From<char> for Unit {
    fn from(char: char) -> Self {
        match char {
            '0' => Unit(0),
            '1' => Unit(1),
            _ => panic!("Expected either a 1 or 0."),
        }
    }
}


fn mode(vec: &Vec<Unit>) -> usize {
    let mut zeros: usize = 0;
    let mut ones: usize = 0;
    for value in vec {
        zeros += (**value == 0) as usize;
        ones += (**value == 1) as usize;
    }

    if zeros > ones { 0 } else { 1 }
}

fn main() {
    // Get the input from a file. 
    // Far from a reasonable use of a build script, but I wanted to see if I could.
    let out_dir = Path::new(env!("OUT_DIR"));
    let path = out_dir.join("binary.txt");
    let file = File::open(path).expect("Expected a file: binary.txt");
    let reader = BufReader::new(file);

    let mut binary: Vec<Vec<Unit>> = Vec::new();
    let mut input = reader.lines();

    if let Some(line) = input.next() {
        let line = line.unwrap();
        let chars = line.chars();
        chars.for_each(|char| binary.push(vec!(Unit::from(char))));
    }

    // Read in the rest of the binary 
    input.for_each(|line| {
        let line = line.unwrap();
        let chars = line.chars();

        chars.enumerate().for_each(|(idx, char)| {
            binary[idx].push(Unit::from(char))
        })
    });

    // Evaluate the data 
    let mut gamma: usize = 0b0;
    let mut epsilon: usize = 0b0;

    for column in binary {
        let common = mode(&column);
        gamma <<= 1;
        epsilon <<= 1;
        gamma += common;
        epsilon += !common & 1;
    }

    println!("Gamma   = {:b}", gamma);
    println!("Epsilon = {:b}", epsilon);
    println!("Power consumption = {}", gamma * epsilon);
}
