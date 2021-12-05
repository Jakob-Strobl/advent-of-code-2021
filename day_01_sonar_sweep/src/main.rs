use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

struct Sonar {
    previous_measurement: isize,
    num_depth_increments: isize,
}
 
impl Sonar {
    fn new() -> Sonar {
        Sonar {
            previous_measurement: isize::MAX,
            num_depth_increments: 0,
        }
    }

    fn scan(mut self, measurement: isize) -> Sonar {
        if measurement > self.previous_measurement {
            // depth is greater!
            self.num_depth_increments += 1;
        }

        self.previous_measurement = measurement;
        return self
    }
}

// Assumes everything is how it should be. Unwraps everything :D
fn main() {
    // Get the file of measurements. 
    // Far from a reasonable use of a build script, but I wanted to see if I could.
    let out_dir = Path::new(env!("OUT_DIR"));
    let measurements_path = out_dir.join("measurements.txt");
    let file = File::open(measurements_path).unwrap();
    let reader = BufReader::new(file);

    // Let's keep it an iterator for the sake of generics 
    let depth_reader = reader.lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .fold(Sonar::new(), Sonar::scan);

    println!("The depth increased {} times!", depth_reader.num_depth_increments)
}