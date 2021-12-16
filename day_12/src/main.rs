use anyhow::Result;
use problem::{solve_main, Problem};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Node {
    is_big: bool,
    edges: Vec<usize>,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    start: usize,
    end: usize,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: vec![
                Node {
                    is_big: false,
                    edges: Vec::new(),
                },
                Node {
                    is_big: false,
                    edges: Vec::new(),
                },
            ],
            start: 0,
            end: 1,
        }
    }

    fn insert_node(&mut self, is_big: bool) -> usize {
        let result = self.nodes.len();
        self.nodes.push(Node {
            is_big,
            edges: Vec::new(),
        });
        result
    }

    fn insert_edge(&mut self, from: usize, to: usize) {
        self.nodes[from].edges.push(to);
        self.nodes[to].edges.push(from);
    }
}

fn is_uppercase(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}

impl problem::Input for Graph {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut name_to_node = HashMap::new();
        let mut result = Self::new();
        name_to_node.insert("start".to_string(), result.start);
        name_to_node.insert("end".to_string(), result.end);
        for line in reader.lines() {
            let line = line?;
            let mut nodes = line.split('-');
            let a = nodes.next().unwrap();
            let b = nodes.next().unwrap();
            let a_index = *name_to_node
                .entry(a.to_string())
                .or_insert_with(|| result.insert_node(is_uppercase(a)));
            let b_index = *name_to_node
                .entry(b.to_string())
                .or_insert_with(|| result.insert_node(is_uppercase(b)));
            result.insert_edge(a_index, b_index);
        }
        Ok(result)
    }
}

struct Day12;

impl Problem for Day12 {
    type Input = Graph;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut paths = VecDeque::new();
        paths.push_back(vec![input.start]);
        let mut count = 0;
        while let Some(path) = paths.pop_front() {
            let current = *path.last().unwrap();
            if current == input.end {
                count += 1;
            } else {
                for &next in input.nodes[current].edges.iter() {
                    if input.nodes[next].is_big || !path.contains(&next) {
                        let mut new = path.clone();
                        new.push(next);
                        paths.push_back(new);
                    }
                }
            }
        }
        count
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut paths = VecDeque::new();
        paths.push_back((vec![input.start], false));
        let mut count = 0;
        while let Some((path, revisited)) = paths.pop_front() {
            let current = *path.last().unwrap();
            if current == input.end {
                count += 1;
            } else {
                for &next in input.nodes[current].edges.iter() {
                    let is_revisit = !input.nodes[next].is_big && path.contains(&next);
                    if next != input.start && (!is_revisit || !revisited) {
                        let mut new = path.clone();
                        new.push(next);
                        paths.push_back((new, revisited || is_revisit));
                    }
                }
            }
        }
        count
    }
}

fn main() {
    solve_main::<Day12>();
}
