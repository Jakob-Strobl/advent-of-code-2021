use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[allow(non_camel_case_types)]
type unit = usize;

#[derive(Debug)]
enum Command {
    Up(unit), // Depth decreases
    Down(unit), // Depth increases
    Forward(unit),
    // Can't reverse 
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        let mut command = input.split_whitespace();

        let direction = command.next().expect("Expected a direction. Found none.");
        let amount = command.next().expect("Expected a unit of travel after a direction.");
        let amount = amount.parse::<unit>().expect("Expected the unit of travel to be a positive integer.");

        let command = match direction {
            "up" => Command::Up(amount),
            "down" => Command::Down(amount),
            "forward" => Command::Forward(amount),
            _ => panic!("Expected a valid direction: forward, up, down. Found {}", direction)
        };

        return command
    }
}

struct Position {
    horizontal: unit,
    depth: unit,
    aim: unit,
}

impl Position {
    fn new() -> Position {
        Position { 
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn apply_command(&mut self, command: Command) {
        match command {
            Command::Up(amount) => self.aim -= amount,
            Command::Down(amount) => self.aim += amount,
            Command::Forward(amount) => {
                self.horizontal += amount;
                self.depth += amount * self.aim;
            },
        }
    }
}

fn main() {
    // Get the input from a file. 
    // Far from a reasonable use of a build script, but I wanted to see if I could.
    let out_dir = Path::new(env!("OUT_DIR"));
    let commands_path = out_dir.join("commands.txt");
    let file = File::open(commands_path).unwrap();
    let reader = BufReader::new(file);

    let commands = reader
        .lines()
        .map(|line| Command::from(line.unwrap().as_str()));

    let mut position = Position::new();

    commands.for_each(|command| position.apply_command(command));

    println!("Position: (horizontal: {}, depth: {}) = {}", position.horizontal, position.depth, position.horizontal * position.depth)
}
