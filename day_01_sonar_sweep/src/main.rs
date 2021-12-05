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

    fn analyze_depth(mut self, measurement: &isize) -> Sonar {
        if *measurement > self.previous_measurement {
            // depth is greater!
            self.num_depth_increments += 1;
        }

        self.previous_measurement = *measurement;
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

    // Parse input into a vector of integers 
    let measurements = reader
        .lines()
        .map(|line| line.unwrap().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    // Part 1
    let raw_sonar = measurements
        .iter()
        .fold(Sonar::new(), |sonar: Sonar, v: &isize | sonar.analyze_depth(&v));

    println!("Number of depth increments:");
    println!("  raw: {}", raw_sonar.num_depth_increments);

    // Part 2: Denoise 
    let measurements = measurements 
        .iter()
        .enumerate()
        .map(|(idx, measurement)| denoise(&measurements, idx, measurement));

    let denoised_sonar = measurements.fold(Sonar::new(), |sonar, v| sonar.analyze_depth(&v));
    println!("  denoised: {}", denoised_sonar.num_depth_increments);
}