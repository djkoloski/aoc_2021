use ::anyhow::Result;
use ::core::iter::Iterator;
use problem::{solve_main, Problem};

struct Points {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Iterator for Points {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= self.height {
            None
        } else {
            let result = (self.x, self.y);
            self.x += 1;
            Some(result)
        }
    }
}

struct Input {
    width: i32,
    height: i32,
    values: Vec<u8>,
}

impl Input {
    fn bounds_check(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if self.bounds_check(x, y) {
            Some(self.values[(x + y * self.width) as usize])
        } else {
            None
        }
    }

    fn points(&self) -> Points {
        Points {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut width = 0;
        let mut height = 0;
        let mut values = Vec::new();
        for line in reader.lines() {
            let line = line?;
            for c in line.chars() {
                values.push(c as u8 - b'0');
            }
            width = line.len() as i32;
            height += 1;
        }
        Ok(Self {
            width,
            height,
            values,
        })
    }
}

const NEIGHBORS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Day9;

impl Problem for Day9 {
    type Input = Input;
    type PartOne = i32;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        input
            .points()
            .filter_map(|(x, y)| {
                let value = input.get(x, y).unwrap();
                NEIGHBORS
                    .iter()
                    .filter_map(|(dx, dy)| input.get(x + dx, y + dy))
                    .all(|n| n > value)
                    .then(|| value as i32 + 1)
            })
            .sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut basins = vec![usize::MAX; (input.width * input.height) as usize];
        let mut counts = Vec::new();
        for (i, (x, y)) in input
            .points()
            .filter(|&(x, y)| {
                NEIGHBORS
                    .iter()
                    .filter_map(|(dx, dy)| input.get(x + dx, y + dy))
                    .all(|n| n > input.get(x, y).unwrap())
            })
            .enumerate()
        {
            basins[(x + y * input.width) as usize] = i;
            counts.push(1);
        }

        let mut finished = false;
        while !finished {
            finished = true;
            for (x, y) in input.points() {
                let index = (x + y * input.width) as usize;
                if basins[(x + y * input.width) as usize] != usize::MAX {
                    for (nx, ny) in NEIGHBORS
                        .iter()
                        .map(|(dx, dy)| (x + dx, y + dy))
                        .filter(|&(nx, ny)| input.get(nx, ny).map(|v| v != 9) == Some(true))
                    {
                        let neighbor_index = (nx + ny * input.width) as usize;
                        if basins[neighbor_index] != basins[index] {
                            basins[neighbor_index] = basins[index];
                            finished = false;
                            counts[basins[index]] += 1;
                        }
                    }
                }
            }
        }

        counts.sort_unstable();
        counts[counts.len() - 3..].iter().product()
    }
}

fn main() {
    solve_main::<Day9>();
}
