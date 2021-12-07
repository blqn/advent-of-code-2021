use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y) = input.split_once(",").ok_or(())?;
        Ok(Self {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
        })
    }
}

#[derive(Debug, Clone)]
struct Line {
    from: Point,
    to: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (from, to) = input.split_once(" -> ").ok_or(())?;
        Ok(Self {
            from: from.parse()?,
            to: to.parse()?,
        })
    }
}

impl Line {
    fn is_inline(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }
}

#[derive(Debug)]
struct Lines(Vec<Line>);

impl Index<usize> for Lines {
    type Output = Line;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

// Conversion from string to puzzle input
impl FromStr for Lines {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Result::from_iter(content.lines().map(|line| line.parse())).map(Self)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Lines = std::fs::read_to_string("inputs/day5.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn part_1(input: &Lines) -> usize {
    input
        .0
        .iter()
        .filter(|line| line.is_inline())
        .flat_map(|line| {
            let Line { from, to } = line;
            let mut v = vec![];
            for i in from.x.min(to.x)..from.x.max(to.x) + 1 {
                for j in from.y.min(to.y)..from.y.max(to.y) + 1 {
                    v.push(Point { x: i, y: j })
                }
            }
            v
        })
        .fold(HashMap::new(), |mut acc, x| {
            match acc.get_mut(&x) {
                Some(count) => *count += 1,
                _ => {
                    acc.insert(x, 1);
                }
            }
            acc
        })
        .iter()
        .filter(|(_, nbr)| **nbr >= 2)
        .count()
}

fn part_2(input: &Lines) -> usize {
    input
        .0
        .iter()
        .flat_map(|line| {
            let Line { from, to } = line;
            let mut v = vec![];
            if !line.is_inline() {
                let slopx = if from.x > to.x { -1 } else { 1 };
                let slopy = if from.y > to.y { -1 } else { 1 };
                let size = (to.x - from.x).abs();
                for i in 0..=size {
                    v.push(Point {
                        x: from.x + (i * slopx),
                        y: from.y + (i * slopy),
                    });
                }
            } else {
                for i in from.x.min(to.x)..=from.x.max(to.x) {
                    for j in from.y.min(to.y)..=from.y.max(to.y) {
                        v.push(Point { x: i, y: j })
                    }
                }
            }
            v
        })
        .fold(HashMap::new(), |mut acc, x| {
            match acc.get_mut(&x) {
                Some(count) => *count += 1,
                _ => {
                    acc.insert(x, 1);
                }
            }
            acc
        })
        .iter()
        .filter(|(_, nbr)| **nbr >= 2)
        .count()
}
