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

    fn analyze_depth(mut self, measurement: isize) -> Sonar {
        if measurement > self.previous_measurement {
            // depth is greater!
            self.num_depth_increments += 1;
        }

        self.previous_measurement = measurement;
        return self
    }
}

fn denoise(measurements: &Vec<isize>, idx: usize, measurement: &isize) -> isize {
    return measurement + measurements.get(idx + 1).unwrap_or(&0) + measurements.get(idx + 2).unwrap_or(&0);
}

// Assumes everything is how it should be. Unwraps everything :D
fn main() {
    // Get the file of measurements. 
    // Far from a reasonable use of a build script, but I wanted to see if I could.
    let out_dir = Path::new(env!("OUT_DIR"));
    let measurements_path = out_dir.join("measurements.txt");
    let file = File::open(measurements_path).unwrap();
    let reader = BufReader::new(file);

    let measurements = reader
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap());

    // Part 2: Denoise 
    // Easily comment our this section for part 1 results
    let measurements = measurements.collect::<Vec<isize>>();
    let measurements = measurements
        .iter()
        .enumerate()
        .map(|(idx, measurement)| denoise(&measurements, idx, measurement));

    let sonar = measurements.fold(Sonar::new(), Sonar::analyze_depth);

    println!("The depth increased {} times!", sonar.num_depth_increments)
}