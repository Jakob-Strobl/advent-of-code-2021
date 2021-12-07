use std::ops::Deref;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
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


fn mode(rows: &Vec<Vec<Unit>>, column: usize) -> usize {
    let mut zeros: usize = 0;
    let mut ones: usize = 0;

    for row in rows {
        let bit = &row[column];
        zeros += (**bit == 0) as usize; // defef and copy from borrowed - i think thats how it works  
        ones += (**bit == 1) as usize;
    }

    if ones >= zeros { 1 } else { 0 }
}

fn mode_2(rows: &[&Vec<Unit>], column: usize) -> usize {
    let mut zeros: usize = 0;
    let mut ones: usize = 0;

    for row in rows {
        let bit = &row[column];
        zeros += (**bit == 0) as usize; // defef and copy from borrowed - i think thats how it works  
        ones += (**bit == 1) as usize;
    }

    if ones >= zeros { 1 } else { 0 }
}

fn main() {
    // Get the input from a file. 
    // Far from a reasonable use of a build script, but I wanted to see if I could.
    let out_dir = Path::new(env!("OUT_DIR"));
    let path = out_dir.join("binary.txt");
    let file = File::open(path).expect("Expected a file: binary.txt");
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<Unit>> = Vec::new();
    let input = reader.lines();

    input.for_each(|line| {
        let line = line.unwrap();
        let chars = line.chars();
        let mut row: Vec<Unit> = Vec::new();

        chars.for_each(|char| row.push(Unit::from(char)));
        rows.push(row);
    });

    let width = rows.get(0).expect("Expected at least one row of data").len();

    // Part 1
    // Calculate gamma and epsilon
    let mut gamma: usize = 0b0;
    let mut epsilon: usize = 0b0;

    for column in 0..width {
        let common = mode(&rows, column);
        gamma <<= 1;
        epsilon <<= 1;
        gamma += common;
        epsilon += !common & 1;
    }

    println!("Gamma   = {:b}", gamma);
    println!("Epsilon = {:b}", epsilon);
    println!("Power consumption = {}", gamma * epsilon);

    // Part 2
    // Oxygen keeps the most common column value
    // CO2 keeps the least common column value
    let mut oxygen: Vec<&Vec<Unit>> = rows.iter().collect(); // Get references to each row
    let mut co2: Vec<&Vec<Unit>> = rows.iter().collect();


    for column in 0..width {
        // Literally could not figure out a nice way with filter or retain
        // Drain filter would be nice, but I'm not using nightly.
        if oxygen.len() > 1 {
            let common = mode_2(oxygen.as_slice(), column);

            oxygen = oxygen.iter().fold(Vec::new(), |mut accum, row| {
                if *row[column] as usize == common {
                    accum.push(row)
                }

                return accum
            });
        }

        if co2.len() > 1 {
            let common = mode_2(co2.as_slice(), column);
            let uncommon = !common & 1;

            co2 = co2.iter().fold(Vec::new(), |mut accum, row| {
                if *row[column] as usize == uncommon {
                    accum.push(row)
                }

                return accum
            });
        }
    }
    
    let mut oxygen_rating = 0b0;
    let mut co2_rating = 0b0;
    for (oxygen, co2) in oxygen[0].iter().zip(co2[0]) {
        oxygen_rating <<= 1;
        co2_rating <<= 1;

        oxygen_rating += **oxygen as usize;
        co2_rating += **co2 as usize;
    }

    println!("O2 Generator Rating = {:b}", oxygen_rating);
    println!("C02 Scrubber Rating = {:b}", co2_rating);
    println!("Life Support Rating = {}", oxygen_rating * co2_rating);
}
