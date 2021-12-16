use ::core::cmp::Reverse;
use ::std::collections::BinaryHeap;
use anyhow::Result;
use problem::{solve_main, Problem};

struct Grid {
    size: usize,
    values: Vec<usize>,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            size,
            values: vec![usize::MAX; size * size],
        }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.values[x + y * self.size]
    }

    fn get_repeated(&self, x: usize, y: usize) -> usize {
        (self.values[(x % self.size) + (y % self.size) * self.size] + x / self.size + y / self.size
            - 1)
            % 9
            + 1
    }

    fn set(&mut self, x: usize, y: usize, value: usize) {
        self.values[x + y * self.size] = value;
    }
}

impl problem::Input for Grid {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines().peekable();
        let mut result = Grid::new(lines.peek().unwrap().as_ref().unwrap().len());
        for (y, line) in lines.enumerate() {
            for (x, c) in line?.chars().enumerate() {
                result.set(x, y, (c as u8 - b'0') as usize);
            }
        }
        Ok(result)
    }
}

struct Day15;

impl Problem for Day15 {
    type Input = Grid;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut risk = Grid::new(input.size);
        let mut frontier = BinaryHeap::new();
        frontier.push((Reverse(0), (0, 0)));
        while let Some((new_risk, (x, y))) = frontier.pop() {
            let current_risk = risk.get(x, y);
            if new_risk.0 < current_risk {
                risk.set(x, y, new_risk.0);
                if x > 0 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x - 1, y)),
                        (x - 1, y),
                    ));
                }
                if x < risk.size - 1 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x + 1, y)),
                        (x + 1, y),
                    ));
                }
                if y > 0 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x, y - 1)),
                        (x, y - 1),
                    ));
                }
                if y < risk.size - 1 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x, y + 1)),
                        (x, y + 1),
                    ));
                }
            }
        }

        risk.get(risk.size - 1, risk.size - 1)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut risk = Grid::new(input.size * 5);
        let mut frontier = BinaryHeap::new();
        frontier.push((Reverse(0), (0, 0)));
        while let Some((new_risk, (x, y))) = frontier.pop() {
            let current_risk = risk.get(x, y);
            if new_risk.0 < current_risk {
                risk.set(x, y, new_risk.0);
                if x > 0 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x - 1, y)),
                        (x - 1, y),
                    ));
                }
                if x < risk.size - 1 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x + 1, y)),
                        (x + 1, y),
                    ));
                }
                if y > 0 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x, y - 1)),
                        (x, y - 1),
                    ));
                }
                if y < risk.size - 1 {
                    frontier.push((
                        Reverse(new_risk.0 + input.get_repeated(x, y + 1)),
                        (x, y + 1),
                    ));
                }
            }
        }

        risk.get(risk.size - 1, risk.size - 1)
    }
}

fn main() {
    solve_main::<Day15>();
}
