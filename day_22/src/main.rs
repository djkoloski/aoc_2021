use ::std::{iter::IntoIterator, str::FromStr};
use ::anyhow::{anyhow, Error, Result};
use ::bitvec::prelude::*;
use problem::{solve_main, Problem};

#[derive(Clone, Copy, Default)]
struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    fn and(self, other: Self) -> Option<Self> {
        let lower = i32::max(self.lower, other.lower);
        let upper = i32::min(self.upper, other.upper);
        if lower > upper {
            None
        } else {
            Some(Self {
                lower,
                upper,
            })
        }
    }
}

impl IntoIterator for Range {
    type Item = i32;
    type IntoIter = ::std::ops::RangeInclusive<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.lower..=self.upper
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut pieces = s.split("..");
        let lower = pieces.next().ok_or(anyhow!("Missing lower bound"))?.parse()?;
        let upper = pieces.next().ok_or(anyhow!("Missing lower bound"))?.parse()?;
        Ok(Self {
            lower,
            upper,
        })
    }
}

#[derive(Clone, Copy, Default)]
struct Region {
    ranges: [Range; 3],
}

impl Region {
    fn and(self, other: Self) -> Option<Self> {
        self.ranges[0].and(other.ranges[0]).and_then(|x| {
            self.ranges[1].and(other.ranges[1]).and_then(|y| {
                self.ranges[2].and(other.ranges[2]).map(|z| Self {
                    ranges: [x, y, z],
                })
            })
        })
    }
}

impl FromStr for Region {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        const PREFIX: [&'static str; 3] = ["x=", "y=", "z="];

        let mut ranges = [Range::default(); 3];
        for (i, piece) in s.split(',').enumerate() {
            if let Some(range) = piece.strip_prefix(PREFIX[i]) {
                ranges[i] = range.parse()?;
            }
        }

        Ok(Self {
            ranges,
        })
    }
}

struct Operation {
    value: bool,
    region: Region,
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (value, rest) =
            s.strip_prefix("on ").map(|rest| (true, rest))
            .or_else(|| s.strip_prefix("off ").map(|rest| (false, rest)))
            .ok_or(anyhow!("Invalid operation value, expected 'on' or 'off'"))?;

        let region = rest.parse()?;

        Ok(Self {
            value,
            region,
        })
    }
}

fn count_ones(operations: &[Operation]) -> usize {
    let mut breakpoints = [Vec::new(), Vec::new(), Vec::new()];
    for i in 0..3 {
        breakpoints[i] = operations.iter().map(|o| [o.region.ranges[i].lower, o.region.ranges[i].upper + 1]).flatten().collect();
        breakpoints[i].sort_unstable();
        breakpoints[i].dedup();
    }

    let size_x = breakpoints[0].len() - 1;
    let size_y = breakpoints[1].len() - 1;
    let size_z = breakpoints[2].len() - 1;
    let area = size_x * size_y * size_z;
    let mut grid = bitvec![0; area];

    for operation in operations.iter() {
        let op_x_lower = breakpoints[0].binary_search(&operation.region.ranges[0].lower).unwrap();
        let op_x_upper = breakpoints[0].binary_search(&(operation.region.ranges[0].upper + 1)).unwrap();
        let op_y_lower = breakpoints[1].binary_search(&operation.region.ranges[1].lower).unwrap();
        let op_y_upper = breakpoints[1].binary_search(&(operation.region.ranges[1].upper + 1)).unwrap();
        let op_z_lower = breakpoints[2].binary_search(&operation.region.ranges[2].lower).unwrap();
        let op_z_upper = breakpoints[2].binary_search(&(operation.region.ranges[2].upper + 1)).unwrap();
        for z in op_z_lower..op_z_upper {
            for y in op_y_lower..op_y_upper {
                for x in op_x_lower..op_x_upper {
                    let index = x + size_x * (y + size_y * z);
                    grid.set(index, operation.value);
                }
            }
        }
    }

    let mut total = 0;
    for z in 0..size_z {
        for y in 0..size_y {
            for x in 0..size_x {
                let index = x + size_x * (y + size_y * z);
                if grid[index] {
                    total +=
                        (breakpoints[0][x + 1] - breakpoints[0][x]) as usize
                        * (breakpoints[1][y + 1] - breakpoints[1][y]) as usize
                        * (breakpoints[2][z + 1] - breakpoints[2][z]) as usize;
                }
            }
        }
    }

    total
}

struct Day22;

impl Problem for Day22 {
    type Input = Vec<Operation>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        const INIT_REGION: Region = Region {
            ranges: [
                Range { lower: -50, upper: 50 },
                Range { lower: -50, upper: 50 },
                Range { lower: -50, upper: 50 },
            ],
        };
        let mut ops = Vec::new();
        for op in input {
            if let Some(region) = op.region.and(INIT_REGION) {
                ops.push(Operation {
                    value: op.value,
                    region,
                });
            }
        }

        count_ones(&ops)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        count_ones(&input)
    }
}

fn main() {
    solve_main::<Day22>();
}
