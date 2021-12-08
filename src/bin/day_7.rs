use std::num::ParseIntError;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct Input(Vec<i32>);

impl Index<usize> for Input {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Result::from_iter(content.trim().split(",").map(|num| num.parse())).map(Self)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day7.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}


fn part_1(input: &Input) -> i32 {
    (0..*input.0.iter().max().unwrap()).map(|i| {
        let sum = input.0.iter().fold(0, |acc, x| (x - i as i32).abs() + acc);
        sum
    }).min().unwrap()
}

fn sum(n: i32) -> i32 {
    (n * (n+1)) / 2
}

fn part_2(input: &Input) -> i32 {
    (0..*input.0.iter().max().unwrap()).map(|i| {
        let sum = input.0.iter().fold(0, |acc, x| sum((x - i as i32).abs()) + acc);
        sum
    }).min().unwrap()
}
