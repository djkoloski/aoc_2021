use ::anyhow::{anyhow, Error, Result};
use ::core::{
    ops::{Add, Sub},
    str::FromStr,
};
use ::std::collections::HashMap;
use problem::{solve_main, Problem};
use std::collections::VecDeque;

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn rotate(self, axis: Axis) -> Self {
        match axis {
            Axis::X => self.rotate_x(),
            Axis::Y => self.rotate_y(),
            Axis::Z => self.rotate_z(),
        }
    }

    pub fn rotate_x(self) -> Self {
        Self {
            x: self.x,
            y: self.z,
            z: -self.y,
        }
    }

    pub fn rotate_y(self) -> Self {
        Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    pub fn rotate_z(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }
}

impl Vector {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl FromStr for Vector {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(',');
        Ok(Self {
            x: components
                .next()
                .ok_or(anyhow!("Missing X component"))?
                .parse()?,
            y: components
                .next()
                .ok_or(anyhow!("Missing Y component"))?
                .parse()?,
            z: components
                .next()
                .ok_or(anyhow!("Missing Z component"))?
                .parse()?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Scanner {
    beacons: Vec<Vector>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            beacons: Vec::new(),
        }
    }

    fn try_orient(&self, other: &mut Scanner) -> Option<Vector> {
        const ROTATION_PATH: [Axis; 6] = [Axis::X, Axis::X, Axis::Z, Axis::X, Axis::X, Axis::Z];

        for axis in ROTATION_PATH {
            for _ in 0..4 {
                let mut offset_counts = HashMap::new();
                for &a in self.beacons.iter() {
                    for &b in other.beacons.iter() {
                        *offset_counts.entry(a - b).or_insert(0) += 1;
                    }
                }
                if let Some(offset) = offset_counts
                    .iter()
                    .filter_map(|(&o, &n)| if n >= 12 { Some(o) } else { None })
                    .next()
                {
                    return Some(offset);
                }
                other.beacons.iter_mut().for_each(|p| *p = p.rotate_y());
            }
            other.beacons.iter_mut().for_each(|p| *p = p.rotate(axis));
        }

        None
    }
}

#[derive(Debug)]
pub struct Input {
    scanners: Vec<Scanner>,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines();
        let mut scanners = Vec::new();
        while let Some(header) = lines.next() {
            let _ = header?;
            let mut scanner = Scanner::new();
            for point in lines.by_ref() {
                let point = point?;
                if point.is_empty() {
                    break;
                }
                scanner.beacons.push(point.parse()?);
            }
            scanners.push(scanner);
        }

        Ok(Self { scanners })
    }
}

struct Day19;

impl Problem for Day19 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = i32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut unoriented = input
            .scanners
            .iter()
            .cloned()
            .map(|s| (s, 0))
            .collect::<VecDeque<_>>();

        let mut oriented = vec![(unoriented.pop_front().unwrap().0, Vector::ZERO)];
        'orient: while let Some((mut u, n)) = unoriented.pop_front() {
            for i in n..oriented.len() {
                let (o, offset) = &oriented[i];
                let offset = *offset;
                if let Some(rel_offset) = o.try_orient(&mut u) {
                    oriented.push((u, offset + rel_offset));
                    continue 'orient;
                }
            }
            unoriented.push_back((u, oriented.len()));
        }

        let mut beacons = Vec::new();
        for (s, o) in oriented.iter() {
            for &b in s.beacons.iter() {
                beacons.push(b + *o);
            }
        }

        beacons.sort_unstable();
        beacons.dedup();

        beacons.len()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut unoriented = input
            .scanners
            .iter()
            .cloned()
            .map(|s| (s, 0))
            .collect::<VecDeque<_>>();

        let mut oriented = vec![(unoriented.pop_front().unwrap().0, Vector::ZERO)];
        'orient: while let Some((mut u, n)) = unoriented.pop_front() {
            for i in n..oriented.len() {
                let (o, offset) = &oriented[i];
                let offset = *offset;
                if let Some(rel_offset) = o.try_orient(&mut u) {
                    oriented.push((u, offset + rel_offset));
                    continue 'orient;
                }
            }
            unoriented.push_back((u, oriented.len()));
        }

        let mut max = 0;
        for i in 0..oriented.len() {
            for j in i + 1..oriented.len() {
                let d = oriented[i].1 - oriented[j].1;
                max = i32::max(max, d.x.abs() + d.y.abs() + d.z.abs());
            }
        }

        max
    }
}

fn main() {
    solve_main::<Day19>();
}
