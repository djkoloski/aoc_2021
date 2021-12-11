use anyhow::Result;
use problem::{solve_main, Problem};

#[derive(Clone)]
struct Grid {
    values: [u8; 100],
}

impl Grid {
    fn update(&mut self) -> usize {
        for y in 0..10 {
            for x in 0..10 {
                self.values[x + y * 10] += 1;
            }
        }

        let mut finished = false;
        while !finished {
            finished = true;
            for y in 0..10 {
                for x in 0..10 {
                    let value = self.values[x as usize + y as usize * 10];
                    if value > 9 && value < 100 {
                        self.values[x as usize + y as usize * 10] = 100;
                        finished = false;
                        for ny in isize::max(0, y - 1)..=isize::min(9, y + 1) {
                            for nx in isize::max(0, x - 1)..=isize::min(9, x + 1) {
                                self.values[nx as usize + ny as usize * 10] += 1;
                            }
                        }
                    }
                }
            }
        }

        let mut flashes = 0;
        for y in 0..10 {
            for x in 0..10 {
                if self.values[x + y * 10] >= 100 {
                    flashes += 1;
                    self.values[x + y * 10] = 0;
                }
            }
        }
        flashes
    }
}

impl problem::Input for Grid {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut result = Grid { values: [0; 100] };
        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line?.chars().enumerate() {
                result.values[x + y * 10] = c as u8 - b'0';
            }
        }
        Ok(result)
    }
}

struct Day11;

impl Problem for Day11 {
    type Input = Grid;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut grid = input.clone();
        let mut flashes = 0;
        for _ in 0..100 {
            flashes += grid.update();
        }
        flashes
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut grid = input.clone();
        let mut i = 1;
        loop {
            if grid.update() == 100 {
                break i;
            }
            i += 1;
        }
    }
}

fn main() {
    solve_main::<Day11>();
}
