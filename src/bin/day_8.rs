use std::collections::HashSet;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct Input(Vec<(Vec<String>, Vec<String>)>);

impl Index<usize> for Input {
    type Output = (Vec<String>, Vec<String>);

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Result::from_iter(content.lines().map(|line| {
            let (pattern, output) = line.split_once(" | ").ok_or(())?;
            Ok((
                pattern.split(" ").map(|x| x.to_string()).collect(),
                output.split(" ").map(|x| x.to_string()).collect(),
            ))
        }))
        .map(Self)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day8.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i32 {
    let lens = HashSet::from([2, 3, 4, 7]);
    input
        .0
        .iter()
        .map(|(_, output)| output.iter().filter(|x| lens.contains(&x.len())).count() as i32)
        .sum::<i32>()
}

fn to_set(digit: String) -> HashSet<char> {
    digit.chars().collect()
}

fn part_2(input: &Input) -> i32 {
    let mut sum: i32 = 0;
    for (pattern, output) in input.0.iter() {
        let mut pattern = pattern.clone();
        pattern.sort_by(|d1, d2| d1.len().cmp(&d2.len()));
        let one = to_set(pattern[0].clone());
        let seven = to_set(pattern[1].clone());
        let four = to_set(pattern[2].clone());
        let eight = to_set(pattern[9].clone());
        let nine = &pattern[6..9]
            .iter()
            .cloned()
            .map(to_set)
            .find(|digit| digit.is_superset(&four))
            .unwrap();
        let five = &pattern[3..9]
            .iter()
            .cloned()
            .map(to_set)
            .find(|digit| digit.is_subset(&nine) && !digit.is_superset(&one))
            .unwrap();
        let six = &pattern[6..9]
            .iter()
            .cloned()
            .map(to_set)
            .find(|digit| digit.is_superset(&five) && digit != nine)
            .unwrap();
        let zero = &pattern[6..9]
            .iter()
            .cloned()
            .map(to_set)
            .find(|digit| !digit.is_superset(&five) && digit != nine)
            .unwrap();
        let three = &pattern[3..9]
            .iter()
            .cloned()
            .map(to_set)
            .find(|digit| digit.is_subset(&nine) && digit != five)
            .unwrap();
        let two = &pattern[3..9]
            .iter()
            .cloned()
            .map(to_set)
            .find(|digit| !digit.is_subset(&nine))
            .unwrap();
        let resolved = vec![
            zero, &one, two, three, &four, five, six, &seven, &eight, nine,
        ];
        sum += output
            .iter()
            .map(|digit| {
                resolved
                    .iter()
                    .enumerate()
                    .find(|(_, x)| **x == &to_set(digit.clone()))
                    .unwrap()
                    .0
                    .to_string()
            })
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
    }
    sum
}
