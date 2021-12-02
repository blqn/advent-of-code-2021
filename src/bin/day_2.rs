use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward
}

impl FromStr for Direction {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "up"  => Ok(Direction::Up),
            "down"  => Ok(Direction::Down),
            "forward"  => Ok(Direction::Forward),
            _      => Err(()),
        }
    }
}

#[derive(Debug)]
struct InputLine(Direction, i32);

impl FromStr for InputLine {

    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
            let split = input.split_once(" ").ok_or(())?;
            let direction: Direction = split.0.parse().map_err(|_| ())?;
            let value: i32 = split.1.parse().map_err(|_| ())?;
            Ok(InputLine(direction, value))
    }
}


#[derive(Debug)]
struct Input(pub Vec<InputLine>);

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Result::from_iter(content.lines().map(|line| line.parse())).map(Input)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day2.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {}", part_1(&input.0));
    println!("solution part 2: {}", part_2(&input.0));
}

fn part_1(input: &Vec<InputLine>) -> i32 {
    input.iter().fold(vec![0, 0], |acc, x| {
        match x.0 {
            Direction::Up => vec![acc[0], acc[1] - x.1],
            Direction::Down => vec![acc[0], acc[1] + x.1],
            Direction::Forward => vec![acc[0] + x.1, acc[1]],
        }
    }).into_iter().reduce(|x, y| x*y).unwrap()
}

fn part_2(input: &Vec<InputLine>) -> i32 {
    input.iter().fold(vec![0, 0, 0], |acc, x| {
        match x.0 {
            Direction::Up => vec![acc[0], acc[1], acc[2] - x.1],
            Direction::Down => vec![acc[0], acc[1], acc[2] + x.1],
            Direction::Forward => vec![acc[0] + x.1,acc[1] + acc[2] * x.1 , acc[2]],
        }
    }).into_iter().take(2).reduce(|x, y| x*y).unwrap()
}
