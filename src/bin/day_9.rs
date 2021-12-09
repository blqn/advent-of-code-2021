use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    height: i32,
    width: i32,
    cave: Vec<i32>,
}

impl Index<(i32, i32)> for Input {
    type Output = i32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        let i: usize = (self.width * index.0 + index.1).try_into().unwrap();
        self.cave.index(i)
    }
}

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let width = content.lines().take(1).collect::<String>().len() as i32;
        let height = content.lines().count() as i32;
        let cave: Vec<i32> = content
            .lines()
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();
        Ok(Self {
            height,
            width,
            cave,
        })
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day9.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn is_lower(input: &Input, i: i32, j: i32) -> bool {
    let adjacents = vec![
        ((i - 1).max(0), j),
        ((i + 1).min(input.height - 1), j),
        (i, (j - 1).max(0)),
        (i, (j + 1).min(input.width - 1)),
    ];
    let mut is_lower = true;
    let point = input[(i, j)];
    for (x, y) in adjacents.iter() {
        if (x, y) != (&i, &j) {
            is_lower = is_lower && point < input[(*x, *y)];
        }
    }
    is_lower
}

fn part_1(input: &Input) -> i32 {
    let mut low = vec![];
    for i in 0..input.height {
        for j in 0..input.width {
            if is_lower(input, i, j) {
                low.push(input[(i, j)]);
            }
        }
    }
    low.iter().sum::<i32>() + low.len() as i32
}

fn part_2(input: &Input) -> i32 {
    let mut bassins = vec![];
    for i in 0..input.height {
        for j in 0..input.width {
            if is_lower(input, i, j) {
                let mut bassin = HashSet::new();
                let mut to_inspect = vec![(i, j)];
                let mut visited = HashSet::new();
                while !to_inspect.is_empty() {
                    let (x, y) = to_inspect.drain(0..1).collect::<Vec<_>>().pop().unwrap();
                    if visited.contains(&(x, y)) {
                        continue;
                    } else {
                        visited.insert((x, y));
                    }
                    if input[(x, y)] != 9 {
                        bassin.insert((x, y));
                    } else {
                        continue;
                    }
                    let ext: Vec<(i32, i32)> = vec![
                        ((x - 1).max(0), y),
                        ((x + 1).min(input.height - 1), y),
                        (x, (y - 1).max(0)),
                        (x, (y + 1).min(input.width - 1)),
                    ]
                    .into_iter()
                    .filter(|p| !visited.contains(&p))
                    .collect();
                    to_inspect.extend(ext);
                }
                bassins.push(bassin.len() as i32);
            }
        }
    }
    bassins.sort();
    bassins.reverse();
    bassins.into_iter().take(3).reduce(|a, b| a * b).unwrap()
}
