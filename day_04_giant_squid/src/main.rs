use std::{str::SplitWhitespace, ops::Add};

#[derive(Debug)]
struct Space {
    number: usize,
    drawn: bool,
}

impl Space {
    fn new(number: usize) -> Space {
        Space { number, drawn: false }
    }

    fn check(&mut self, drawn: usize) {
        if self.number == drawn {
            self.drawn = true;
        }
    }
}

struct Board(Vec<Vec<Space>>);
type Boards = Vec<Board>;

impl Board {
    fn check(&mut self, drawn: usize) {
        for row in &mut self.0 {
            for space in row {
                space.check(drawn);
            }
        }
    }
    
    fn check_win(&self) -> Option<usize> {
        let mut row_count = [0; 5]; // Count of drawn spaces - row
        let mut col_count = [0; 5]; // Count of drawn spaces - column

        // Count row and columns for drawn spaces
        for (ridx, row) in self.0.iter().enumerate() {
            for (cidx, space) in row.iter().enumerate() {
                row_count[ridx] += space.drawn as usize;
                col_count[cidx] += space.drawn as usize;
            }
        }

        // Sum of all unmarked numbers 
        if row_count.contains(&5) || col_count.contains(&5) {
            let mut score = 0;

            for row in &self.0 {
                score += row.iter().fold(0,|mut sum, space| {
                    if !space.drawn {
                        sum += space.number
                    }
                    return sum
                })
            }

            return Some(score);
        }

        return None
    }
}

impl<'a> TryFrom<&mut SplitWhitespace<'a>> for Board {
    type Error = ();

    fn try_from(input: &mut SplitWhitespace) -> Result<Self, Self::Error> {
        let mut table = vec![];

        for _ in 0..5 {
            let row: Vec<Space> = input.take(5)
                .map(|s | {
                    let number = s.parse().expect("Expected a unsized integer");
                    Space::new(number)
                })
                .collect();

            if row.len() == 5 {
                table.push(row);
            } else {
                return Err(()) // EOF
            }
        }

        Ok(Board(table))
    }
}

fn parse_input(input: &str) -> (Vec<usize>, Boards) {
    // First line is drawn numbers 
    let mut input = input.split_whitespace();

    let numbers = input.next().expect("Expected to start with a list of numbers.");
    let numbers: Vec<usize> = numbers.split(',')
        .map(|draw| draw.parse().expect("Expected a unsized integer"))
        .collect();

    let mut boards: Boards = vec![];
    while let Ok(board) = Board::try_from(&mut input) {
        boards.push(board);
    }

    return (numbers, boards)
}

fn play_bingo(numbers: Vec<usize>, mut boards: Boards) -> usize {
    for drawn in numbers {
        for board in boards.iter_mut() {
            board.check(drawn);
            if let Some(score) = board.check_win() {
                return score * drawn
            }
        }
    }

    return 0
}

fn play_bingo_last(numbers: Vec<usize>, mut boards: Boards) -> usize {
    // (bidx, score)
    let mut winning_boards: Vec<(usize, usize)> = Vec::new();

    for drawn in numbers {
        for (bidx, board) in boards.iter_mut().enumerate() {
            board.check(drawn);
            if let Some(score) = board.check_win() {
                if winning_boards.iter().find(|(wbidx, _)| wbidx == &bidx).is_none() {
                    winning_boards.push((bidx, score * drawn));
                }
            }
        }
    }

    return winning_boards.pop().unwrap().1
}

fn main() {
    // Learned about include_str macro :D 
    // Part 1
    let (numbers, boards) = parse_input(include_str!("../input/bingo.txt"));
    let result = play_bingo(numbers, boards);
    println!("Part 1 result: {}", result);

    let (numbers, boards) = parse_input(include_str!("../input/bingo.txt"));
    let result = play_bingo_last(numbers, boards);
    println!("Part 2 result: {}", result);
}

// Unit test on example
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let (drawn, boards) = parse_input(include_str!("../input/sample.txt"));

        let expected = 4512;
        assert_eq!(play_bingo(drawn, boards), expected);
    }

    #[test]
    fn part_two() {
        let (drawn, boards) = parse_input(include_str!("../input/sample.txt"));

        let expected = 1924;
        assert_eq!(play_bingo_last(drawn, boards), expected);
    }
}