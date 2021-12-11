use std::collections::HashSet;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct Input(Vec<String>);

impl Index<usize> for Input {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Result::from_iter(content.lines().map(|line| line.parse().map_err(|_| ()))).map(Input)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day10.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i32 {
    let openers: HashSet<char> = {
        let mut h = HashSet::new();
        h.insert('(');
        h.insert('{');
        h.insert('[');
        h.insert('<');
        h
    };
    input
        .0
        .iter()
        .map(|line| {
            let mut stack = vec![];
            for ch in line.chars() {
                if openers.contains(&ch) {
                    stack.push(ch.clone());
                } else {
                    let last = stack.pop().unwrap();
                    let diff = ch as i32 - last as i32;
                    if diff > 2 || diff < 0 {
                        return Some(ch);
                    }
                }
            }
            None
        })
        .fold(0, |acc, char_opt| {
            if let Some(c) = char_opt {
                (match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                }) + acc
            } else {
                acc
            }
        })
}

fn part_2(input: &Input) -> i64 {
    let openers: HashSet<char> = {
        let mut h = HashSet::new();
        h.insert('(');
        h.insert('{');
        h.insert('[');
        h.insert('<');
        h
    };
    let mut scores = input
        .0
        .iter()
        .filter_map(|line| {
            let mut stack = vec![];
            for ch in line.chars() {
                if openers.contains(&ch) {
                    stack.push(ch.clone());
                } else {
                    let last = stack.pop().unwrap();
                    let diff = ch as i32 - last as i32;
                    if diff > 2 || diff < 0 {
                        return None;
                    }
                }
            }
            stack.reverse();
            let score = stack.iter().fold(0, |acc, c| acc * 5 + (match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _=> 0,
            }));
            Some(score)
        }).collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
}
