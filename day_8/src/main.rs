use std::str::FromStr;
use anyhow::{anyhow, Error, Result};
use problem::{solve_main, Problem};

const A: u8 = 0b0000001;
const B: u8 = 0b0000010;
const C: u8 = 0b0000100;
const D: u8 = 0b0001000;
const E: u8 = 0b0010000;
const F: u8 = 0b0100000;
const G: u8 = 0b1000000;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct Digit(u8);

impl Digit {
    fn set(self) -> usize {
        self.0.count_ones() as usize
    }

    fn matches(self, other: Digit) -> usize {
        (self.0 & other.0).count_ones() as usize
    }
}

impl FromStr for Digit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut result = 0;
        for c in s.chars() {
            result |= match c {
                'a' => A,
                'b' => B,
                'c' => C,
                'd' => D,
                'e' => E,
                'f' => F,
                'g' => G,
                _ => return Err(anyhow!("Invalid display segment: {}", c)),
            }
        }
        Ok(Digit(result))
    }
}

struct Display {
    combinations: [Digit; 10],
    digits: [Digit; 4],
}

const REAL_DIGITS: [Digit; 10] = [
    Digit(A | B | C | E | F | G),
    Digit(C | F),
    Digit(A | C | D | E | G),
    Digit(A | C | D | F | G),
    Digit(B | C | D | F),
    Digit(A | B | D | F | G),
    Digit(A | B | D | E | F | G),
    Digit(A | C | F),
    Digit(A | B | C | D | E | F | G),
    Digit(A | B | C | D | F | G),
];

impl Display {
    fn solve(&self) -> usize {
        let mut candidates = [
            vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![],
        ];
        for (i, &d) in self.combinations.iter().enumerate() {
            let num_set = d.set();
            for (j, &n) in REAL_DIGITS.iter().enumerate() {
                if n.set() == num_set {
                    candidates[i].push(j);
                }
            }
        }

        while candidates.iter().any(|c| c.len() != 1) {
            for i in 0..10 {
                if candidates[i].len() == 1 {
                    let disc_scrambled = self.combinations[i];
                    let disc_real = REAL_DIGITS[candidates[i][0]];
                    for j in 0..10 {
                        let unknown_scrambled = self.combinations[j];
                        let matches = unknown_scrambled.matches(disc_scrambled);
                        candidates[j].retain(|c| REAL_DIGITS[*c].matches(disc_real) == matches);
                    }
                }
            }
        }

        let mut result = 0;
        for i in 0..4 {
            let index = (0..10).find(|&n| self.combinations[n] == self.digits[i]).unwrap();
            result = result * 10 + candidates[index][0];
        }

        result
    }
}

impl FromStr for Display {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut combinations = [Digit::default(); 10];
        let mut digits = [Digit::default(); 4];

        let mut pieces = s.split(" | ");
        for (i, combination) in pieces.next().ok_or(anyhow!("Missing combinations"))?.split(' ').enumerate() {
            combinations[i] = combination.parse()?;
        }
        for (i, digit) in pieces.next().ok_or(anyhow!("Missing digits"))?.split(' ').enumerate() {
            digits[i] = digit.parse()?;
        }
        Ok(Display {
            combinations,
            digits,
        })
    }
}

struct Day8;

impl Problem for Day8 {
    type Input = Vec<Display>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        input.iter().map(|d| d.digits.iter().filter(|d| d.set() == 2 || d.set() == 3 || d.set() == 4 || d.set() == 7 ).count()).sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        input.iter().map(|d| d.solve()).sum()
    }
}

fn main() {
    solve_main::<Day8>();
}
