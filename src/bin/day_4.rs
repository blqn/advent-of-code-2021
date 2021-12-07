use std::iter::FromIterator;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Numbers(Vec<i32>);

impl FromStr for Numbers {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Result::from_iter(input.split(",").map(|x| x.parse()))
            .map_err(|_| ())
            .map(Self)
    }
}

#[derive(Debug, Clone)]
struct BingoLine(Vec<(i32, bool)>);

impl FromStr for BingoLine {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Result::from_iter(
            input
                .split(" ")
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .map(|x| x.parse().map(|r| (r, false))),
        )
        .map_err(|_| ())
        .map(Self)
    }
}

impl Index<usize> for BingoLine {
    type Output = (i32, bool);

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

#[derive(Debug, Clone)]
struct Bingo(pub Vec<BingoLine>);

impl Index<usize> for Bingo {
    type Output = BingoLine;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl Bingo {
    pub fn mark(&mut self, number: i32) {
        for line in &mut self.0 {
            for entry in &mut line.0 {
                entry.1 = entry.1 || entry.0 == number;
            }
        }
    }

    pub fn win(&self) -> bool {
        for i in 0..5 {
            let mut col = true;
            let mut row = true;
            for j in 0..5 {
                row = self[i][j].1 && row;
                col = self[j][i].1 && col;
            }
            if col || row {
                return true;
            }
        }
        false
    }

    pub fn sum_unmarked(&self) -> i32 {
        self.0
            .iter()
            .map(|line| {
                line.0
                    .iter()
                    .fold(0, |acc, (x, c)| if *c { acc } else { acc + x })
            })
            .sum()
    }
}

#[derive(Debug)]
struct Input {
    numbers: Numbers,
    bingos: Vec<Bingo>,
}

// Conversion from string to puzzle input
impl FromStr for Input {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let numbers: Numbers = content.lines().take(1).collect::<String>().parse()?;
        let bingos: Vec<Bingo> = content
            .lines()
            .skip(1)
            .enumerate()
            .fold(Ok((vec![], vec![])), |acc, (i, line)| {
                if let Ok((mut temp, mut res)) = acc {
                    if i % 6 != 0 {
                        let bingo_line: BingoLine = line.parse().map_err(|_| ())?;
                        temp.push(bingo_line);
                    }
                    if temp.len() == 5 {
                        res.push(Bingo(temp));
                        temp = vec![];
                    }
                    Ok((temp, res))
                } else {
                    Err(())
                }
            })
            .map(|x| x.1)?;
        Ok(Input { numbers, bingos })
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Input = std::fs::read_to_string("inputs/day4.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn part_1(input: &Input) -> Option<i32> {
    let mut bingos = input.bingos.clone();
    for number in input.numbers.0.iter() {
        for bingo in bingos.iter_mut() {
            bingo.mark(*number);
        }
        for bingo in bingos.iter() {
            if bingo.win() {
                return Some(bingo.sum_unmarked() * number);
            }
        }
    }
    None
}

fn part_2(input: &Input) -> i32 {
    let mut bingos = input.bingos.clone();
    let mut nbr_iter = input.numbers.0.iter();
    let mut number = None;
    while bingos.len() > 1 {
        number = nbr_iter.next();
        for bingo in bingos.iter_mut() {
            bingo.mark(*number.unwrap());
        }
        bingos = bingos.into_iter().filter(|bingo| !bingo.win()).collect();
    }
    let mut last = bingos[0].to_owned();
    while !last.win() {
        number = nbr_iter.next();
        last.mark(*number.unwrap());
    }
    last.mark(*number.unwrap());
    last.sum_unmarked() * number.unwrap()
}
