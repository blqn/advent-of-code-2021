use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
struct Input {
    height: i32,
    width: i32,
    flash: Vec<i32>,
}

impl Index<(i32, i32)> for Input {
    type Output = i32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        let i: usize = (self.width * index.0 + index.1).try_into().unwrap();
        self.flash.index(i)
    }
}

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ParseIntError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let width = content.lines().take(1).collect::<String>().len() as i32;
        let height = content.lines().count() as i32;
        let flash: Vec<i32> = content
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
            flash,
        })
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day11.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> i32 {
    let (w, h) = (input.width, input.height);
    let mut flash = input.flash.clone();
    let mut count = 0;
    for _step in 0..100 {
        flash = flash.into_iter().map(|level| level + 1).collect();
        let mut to_reset = HashSet::new();
        for i in 0..h {
            for j in 0..w {
                let mut spread = vec![(i, j)];
                while !spread.is_empty() {
                    let (a, b) = spread.pop().unwrap();
                    let level = flash[(w * a + b) as usize];
                    if level <= 9 {
                        continue;
                    } else {
                        to_reset.insert((a, b));
                        flash[(w * a + b) as usize] = 0;
                        for x in 0.max(a - 1)..=(w-1).min(a + 1) {
                            for y in 0.max(b - 1)..=(h-1).min(b + 1) {
                                if (x, y) != (i,j){
                                    flash[(w * x + y) as usize] += 1;
                                    spread.push((x, y));
                                }
                            }
                        }
                    }
                }
            }
        }
        for (i,j) in to_reset.iter() {
            flash[(w * i + j) as usize] = 0;
            count+=1;
        }
    }
    count
}

fn part_2(input: &Input) -> i32 {
    let (w, h) = (input.width, input.height);
    let mut flash = input.flash.clone();
    let mut step = 1;
    loop {
        flash = flash.into_iter().map(|level| level + 1).collect();
        let mut to_reset = HashSet::new();
        for i in 0..h {
            for j in 0..w {
                let mut spread = vec![(i, j)];
                while !spread.is_empty() {
                    let (a, b) = spread.pop().unwrap();
                    let level = flash[(w * a + b) as usize];
                    if level <= 9 {
                        continue;
                    } else {
                        to_reset.insert((a, b));
                        flash[(w * a + b) as usize] = 0;
                        for x in 0.max(a - 1)..=(w-1).min(a + 1) {
                            for y in 0.max(b - 1)..=(h-1).min(b + 1) {
                                if (x, y) != (i,j){
                                    flash[(w * x + y) as usize] += 1;
                                    spread.push((x, y));
                                }
                            }
                        }
                    }
                }
            }
        }
        if to_reset.len() as i32 == w*h {
            break;
        }
        for (i,j) in to_reset.iter() {
            flash[(w * i + j) as usize] = 0;
        }
        step+=1;
    }
    step
}
