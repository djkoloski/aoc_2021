use anyhow::{anyhow, Result};
use problem::{solve_main, Problem};

#[repr(u8)]
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Cell {
    Empty,
    East,
    South,
    EastMove,
    SouthMove,
}

#[derive(Clone)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn index(&self, x: usize, y: usize) -> usize {
        x % self.width + y % self.height * self.width
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.index(x, y);
        self.cells[index] = cell;
    }

    pub fn step(&mut self) -> usize {
        let mut moved = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Cell::East && self.get(x + 1, y) == Cell::Empty {
                    self.set(x, y, Cell::EastMove);
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Cell::EastMove {
                    self.set(x, y, Cell::Empty);
                    self.set(x + 1, y, Cell::East);
                    moved += 1;
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Cell::South && self.get(x, y + 1) == Cell::Empty {
                    self.set(x, y, Cell::SouthMove);
                }
            }
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == Cell::SouthMove {
                    self.set(x, y, Cell::Empty);
                    self.set(x, y + 1, Cell::South);
                    moved += 1;
                }
            }
        }

        moved
    }
}

impl problem::Input for Grid {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut cells = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in reader.lines() {
            let line = line?;
            width = Some(line.len());
            height += 1;
            for c in line.chars() {
                cells.push(match c {
                    '.' => Cell::Empty,
                    '>' => Cell::East,
                    'v' => Cell::South,
                    _ => return Err(anyhow!("Invalid char: '{}'", c)),
                });
            }
        }

        Ok(Grid {
            cells,
            width: width.ok_or(anyhow!("Invalid width"))?,
            height,
        })
    }
}

struct Day25;

impl Problem for Day25 {
    type Input = Grid;
    type PartOne = usize;
    type PartTwo = problem::Unimplemented;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut grid = input.clone();

        let mut i = 1;
        while grid.step() != 0 {
            i += 1;
        }

        i
    }

    fn solve_part_two(_input: &Self::Input) -> Self::PartTwo {
        problem::Unimplemented
    }
}

fn main() {
    solve_main::<Day25>();
}
