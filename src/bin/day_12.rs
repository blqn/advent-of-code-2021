use std::collections::{HashMap, HashSet};
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
pub struct Graph {
    start: NodeIndex,
    end: NodeIndex,
    nodes: Vec<NodeData>,
}

pub type NodeIndex = usize;

#[derive(Debug)]
pub struct NodeData {
    name: String,
    neightbors: Vec<NodeIndex>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            start: 0,
            end: 0,
        }
    }

    fn add_node(&mut self, name: &str) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            neightbors: vec![],
            name: name.to_string(),
        });
        index
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.nodes[from].neightbors.push(to);
    }
}

impl Index<NodeIndex> for Graph {
    type Output = NodeData;

    fn index(&self, index: NodeIndex) -> &Self::Output {
        self.nodes.index(index)
    }
}

// Conversion from string to puzzle input
impl FromStr for Graph {
    type Err = ();

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();
        let mut added = HashMap::new();
        for line in content.lines() {
            let (left, right) = line.split_once("-").ok_or(())?;
            let from = if let Some(from_) = added.get(&left) {
                *from_
            } else {
                let from_ = graph.add_node(left);
                added.insert(left.clone(), from_);
                from_
            };
            let to = if let Some(to_) = added.get(&right) {
                *to_
            } else {
                let to_ = graph.add_node(right);
                added.insert(right.clone(), to_);
                to_
            };
            graph.add_edge(from, to);
            graph.add_edge(to, from);
        }
        for (i, node) in graph.nodes.iter().enumerate() {
            if node.name == "start" {
                graph.start = i;
            }
            if node.name == "end" {
                graph.end = i;
            }
        }
        Ok(graph)
    }
}

/// Main function that loads the input for this day and computes the solution
fn main() {
    let input: Graph = std::fs::read_to_string("inputs/day12.txt")
        .unwrap()
        .parse()
        .unwrap();
    println!("{:?}", input);
    println!("solution part 1: {:?}", part_1(&input));
    println!("solution part 2: {}", part_2(&input));
}

fn part_1(input: &Graph) -> i32 {
    let mut to_visit = vec![(HashSet::new(), input.start)];
    let mut counter = 0;
    while !to_visit.is_empty() {
        let (past, current) = to_visit.pop().unwrap();
        for neightbour_index in input[current].neightbors.iter() {
            if *neightbour_index == input.start {
                continue;
            }
            if *neightbour_index == input.end {
                counter += 1;
                continue;
            }
            let neightbour = &input[*neightbour_index];
            if neightbour.name.to_uppercase() == neightbour.name
                || (neightbour.name.to_lowercase() == neightbour.name
                    && !past.contains(neightbour_index))
            {
                let mut new_past = past.clone();
                new_past.insert(current);
                to_visit.push((new_past, *neightbour_index));
            }
        }
    }
    counter
}

fn part_2(input: &Graph) -> i32 {
    let mut to_visit = vec![(Vec::new(), None, input.start)];
    let mut paths = HashSet::new();
    while !to_visit.is_empty() {
        let (past, twice, current) = to_visit.pop().unwrap();
        if current == input.start && !past.is_empty() {
            continue;
        }
        if current == input.end {
            paths.insert(past);
            continue;
        }
        let node = &input[current];
        let is_upper = node.name.to_uppercase() == node.name;

        let mut new_past = past.clone();
        new_past.push(current);
        let count = past.iter().filter(|x| **x == current).count();
        match (is_upper, count, twice) {
            (true, _, _) => {
                for neightbour in node.neightbors.iter() {
                    to_visit.push((new_past.clone(), twice, *neightbour));
                }
            }
            (_, 0, Some(x)) | (_, 1, Some(x)) if x == current => {
                for neightbour in node.neightbors.iter() {
                    to_visit.push((new_past.clone(), twice, *neightbour));
                }
            }
            (_, 0, Some(x)) if x != current => {
                for neightbour in node.neightbors.iter() {
                    to_visit.push((new_past.clone(), twice, *neightbour));
                }
            }
            (_, 0, None) => {
                for neightbour in node.neightbors.iter() {
                    to_visit.push((new_past.clone(), Some(current), *neightbour));
                    to_visit.push((new_past.clone(), None, *neightbour));
                }
            }
            _ => {}
        }
    }
    paths.len() as i32
}
