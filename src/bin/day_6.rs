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
    let input: Input = std::fs::read_to_string("inputs/day6.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn compute(start: &Vec<i32>, days: i32) -> i64 {
    let mut adults = vec![0; 7];
    let mut babies = vec![0; 2];
    for init in start.iter() {
        let i: usize = (*init).try_into().unwrap();
        adults[i] += 1
    }
    for (i, x) in adults.iter().enumerate() {
        println!("{} - {}", i, x);
    }
    for _ in 0..days {
        let head = adults[0];
        adults = adults[1..].iter().cloned().collect();
        adults.push(head);
        adults[6] += babies[0];
        babies[0] = babies[1];
        babies[1] = head;
    }
    adults.iter().sum::<i64>() + babies.iter().sum::<i64>()
}

fn part_1(input: &Input) -> i64 {
    compute(&input.0, 80)
}

fn part_2(input: &Input) -> i64 {
    compute(&input.0, 256)
}
