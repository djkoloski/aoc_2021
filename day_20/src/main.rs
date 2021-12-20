use std::iter;
use anyhow::Context;
use ::anyhow::{anyhow, Result};
use ::bitvec::prelude::*;
use problem::{solve_main, Problem};

#[derive(Clone)]
pub struct Grid {
    elements: BitVec,
    size: usize,
}

impl Grid {
    #[inline]
    pub fn new(size: usize) -> Self {
        Self {
            elements: bitvec![0; size * size],
            size,
        }
    }

    #[inline]
    pub fn get_any(&self, x: isize, y: isize, default: bool) -> bool {
        if x < 0 || y < 0 || x >= self.size as isize || y >= self.size as isize {
            default
        } else {
            self.get(x as usize, y as usize)
        }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.elements[x + y * self.size]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize) {
        self.elements.set(x + y * self.size, true);
    }

    #[inline]
    pub fn reset(&mut self, x: usize, y: usize) {
        self.elements.set(x + y * self.size, false);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.elements.set_all(false);
    }

    #[inline]
    pub fn neighbor_index(&self, x: usize, y: usize, rest_lit: bool) -> usize {
        let mut result = 0;
        for ny in y as isize - 1..=y as isize + 1 {
            for nx in x as isize - 1..= x as isize + 1 {
                result <<= 1;
                result |= if self.get_any(nx, ny, rest_lit) { 1 } else { 0 };
            }
        }
        result
    }

    pub fn step(&self, lookup: &BitSlice, rest_lit: bool, out: &mut Self) -> bool {
        assert_eq!(self.size, out.size);

        out.clear();

        for y in 0..self.size {
            for x in 0..self.size {
                if lookup[self.neighbor_index(x, y, rest_lit)] {
                    out.set(x, y);
                }
            }
        }

        if rest_lit {
            lookup[0b1_1111_1111]
        } else {
            lookup[0b0_0000_0000]
        }
    }
}

pub struct Input {
    lookup: BitArr!(for 512),
    initial: Grid,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines();

        let mut lookup = bitarr![0; 512];
        for (i, c) in lines.next().ok_or(anyhow!("Missing enhancement algorithm"))??.chars().enumerate() {
            match c {
                '.' => lookup.set(i, false),
                '#' => lookup.set(i, true),
                _ => return Err(anyhow!("Invalid character in enhancement algorithm: {}", c)),
            }
        }

        lines.next().context("Expected line separator between enhancement algorithm and input image")??;

        let first_line = lines.next().context("Missing input image")??;
        let mut initial = Grid::new(first_line.len());
        for (y, line) in iter::once(Ok(first_line)).chain(lines).enumerate() {
            for (x, c) in line?.chars().enumerate() {
                match c {
                    '.' => initial.reset(x, y),
                    '#' => initial.set(x, y),
                    _ => return Err(anyhow!("Invalid character in input image: {}", c)),
                }
            }
        }

        Ok(Self {
            lookup,
            initial,
        })
    }
}

pub fn simulate(input: &Input, steps: usize) -> Grid {
    let border = steps + 1;
    let mut result = Grid::new(input.initial.size + 2 * border);
    let mut output = result.clone();

    // Initial setup
    for x in 0..input.initial.size {
        for y in 0..input.initial.size {
            if input.initial.get(x, y) {
                result.set(x + border, y + border);
            }
        }
    }

    // Update
    let mut rest_lit = false;
    for _ in 0..steps {
        rest_lit = result.step(&input.lookup, rest_lit, &mut output);
        ::core::mem::swap(&mut result, &mut output);
    }

    result
}

struct Day20;

impl Problem for Day20 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        simulate(input, 2).elements.iter().filter(|x| **x).count()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        simulate(input, 50).elements.iter().filter(|x| **x).count()
    }
}

fn main() {
    solve_main::<Day20>();
}
