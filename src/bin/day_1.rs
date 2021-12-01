use std::iter::FromIterator;
use std::str::FromStr;

/// Puzzle input
#[derive(Debug)]
struct Input(pub Vec<i32>);

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = <i32 as FromStr>::Err;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Result::from_iter(content.lines().map(|line| line.parse())).map(Input)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day1.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {}", part_1(&input.0));
    println!("solution part 2: {}", part_2(input.0));
}

fn part_1(input: &Vec<i32>) -> i32 {
    input
        .iter()
        .fold((input[0], 0), |acc, &x| {
            if x > acc.0 {
                (x, acc.1 + 1)
            } else {
                (x, acc.1)
            }
        })
        .1
}

fn part_2(input: Vec<i32>) -> i32 {
    let zero: Vec<i32> = input.iter().take(3).cloned().collect();
    input
        .iter()
        .skip(3)
        .fold((zero, 0), |acc, &x| {
            let next = vec![acc.0[1], acc.0[2], x];
            if next.iter().sum::<i32>() > acc.0.iter().sum() {
                (next, acc.1 + 1)
            } else {
                (next, acc.1)
            }
        })
        .1
}
