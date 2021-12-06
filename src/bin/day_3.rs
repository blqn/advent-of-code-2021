use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Bit {
    Zero,
    One,
}

impl FromStr for Bit {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "0" => Ok(Bit::Zero),
            "1" => Ok(Bit::One),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct InputLine(Vec<Bit>);

impl FromStr for InputLine {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Result::from_iter(input.chars().map(|char| char.to_string().parse())).map(InputLine)
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
    let input: Input = std::fs::read_to_string("inputs/day3.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {}", part_1(&input.0));
    println!("solution part 2: {}", part_2(input.0));
}

fn part_1(input: &Vec<InputLine>) -> i32 {
    let mut gamma = vec![0; input[0].0.len()];
    let mut epsilon = vec![0; input[0].0.len()];
    for x in input.iter() {
        for (i, b) in x.0.iter().enumerate() {
            match b {
                &Bit::One => {
                    gamma[i] += 1;
                    epsilon[i] -= 1;
                }
                &Bit::Zero => {
                    gamma[i] -= 1;
                    epsilon[i] += 1;
                }
            }
        }
    }
    let gamma: i32 = i32::from_str_radix(
        &gamma
            .iter()
            .map(|x| if x > &0 { "1" } else { "0" })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let epsilon: i32 = i32::from_str_radix(
        &epsilon
            .iter()
            .map(|x| if x > &0 { "1" } else { "0" })
            .collect::<String>(),
        2,
    )
    .unwrap();
    gamma * epsilon
}

fn part_2(input: Vec<InputLine>) -> i32 {
    let mut oxy = input.clone();
    let mut i = 0;
    while oxy.len() > 1 || i > input[0].0.len() {
        let mut counter = 0;
        for x in oxy.iter() {
            match x.0[i] {
                Bit::One => {
                    counter += 1;
                }
                Bit::Zero => {
                    counter -= 1;
                }
            }
        }
        let msb = if counter >= 0 { Bit::One } else { Bit::Zero };
        oxy = oxy.into_iter().filter(|x| x.0[i] == msb).collect();
        i += 1;
    }
    let oxy = i32::from_str_radix(
        &oxy[0]
            .0
            .iter()
            .map(|x| if x == &Bit::One { "1" } else { "0" })
            .collect::<String>(),
        2,
    )
    .unwrap();
    let mut co2 = input.clone();
    i = 0;
    while co2.len() > 1 || i > input[0].0.len() {
        let mut counter = 0;
        for x in co2.iter() {
            match x.0[i] {
                Bit::One => {
                    counter -= 1;
                }
                Bit::Zero => {
                    counter += 1;
                }
            }
        }
        let msb = if counter > 0 { Bit::One } else { Bit::Zero };
        co2 = co2.into_iter().filter(|x| x.0[i] == msb).collect();
        i += 1;
    }
    let co2 = i32::from_str_radix(
        &co2[0]
            .0
            .iter()
            .map(|x| if x == &Bit::One { "1" } else { "0" })
            .collect::<String>(),
        2,
    )
    .unwrap();

    co2 * oxy
}
