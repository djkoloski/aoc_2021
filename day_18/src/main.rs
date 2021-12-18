use ::anyhow::{anyhow, Error, Result};
use ::core::{iter::Sum, ops::Add, str::FromStr};
use problem::{solve_main, Problem};
use std::iter::Peekable;

#[derive(Clone, Debug)]
pub enum SFNum {
    Primitive(usize),
    Compound(Box<SFNum>, Box<SFNum>),
}

impl SFNum {
    pub fn reduce(&mut self) {
        while self.explode(0).0 || self.split() {}
    }

    fn leftmost(&mut self) -> &mut usize {
        match self {
            Self::Primitive(x) => x,
            Self::Compound(left, _) => left.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut usize {
        match self {
            Self::Primitive(x) => x,
            Self::Compound(_, right) => right.rightmost(),
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<usize>, Option<usize>) {
        if let Self::Compound(left, right) = self {
            if depth == 4 {
                let (l, r) = match (&**left, &**right) {
                    (Self::Primitive(left), Self::Primitive(right)) => (*left, *right),
                    _ => unreachable!(),
                };
                *self = SFNum::Primitive(0);
                (true, Some(l), Some(r))
            } else if let (true, l, r) = left.explode(depth + 1) {
                if let Some(r) = r {
                    *right.leftmost() += r;
                }
                (true, l, None)
            } else if let (true, l, r) = right.explode(depth + 1) {
                if let Some(l) = l {
                    *left.rightmost() += l;
                }
                (true, None, r)
            } else {
                (false, None, None)
            }
        } else {
            (false, None, None)
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Primitive(x) => {
                if *x >= 10 {
                    *self = Self::Compound(
                        Box::new(Self::Primitive(*x / 2)),
                        Box::new(Self::Primitive((*x + 1) / 2)),
                    );
                    true
                } else {
                    false
                }
            }
            Self::Compound(left, right) => left.split() || right.split(),
        }
    }

    pub fn magnitude(&self) -> usize {
        match self {
            Self::Primitive(x) => *x,
            Self::Compound(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    pub fn parse<I: Iterator<Item = char>>(chars: &mut Peekable<I>) -> Result<Self> {
        match chars.next() {
            None => Err(anyhow!("Unexpected end of input")),
            Some('[') => {
                let left = Self::parse(chars)?;
                match chars.next() {
                    None => return Err(anyhow!("Unexpected end of input")),
                    Some(',') => (),
                    Some(c) => return Err(anyhow!("Expected ',', found '{}'", c)),
                }
                let right = Self::parse(chars)?;
                match chars.next() {
                    None => return Err(anyhow!("Unexpected end of input")),
                    Some(']') => (),
                    Some(c) => return Err(anyhow!("Expected ']', found '{}'", c)),
                }
                Ok(Self::Compound(Box::new(left), Box::new(right)))
            }
            Some(n) => {
                let mut value = n as usize - b'0' as usize;
                while let Some(n @ '0'..='9') = chars.peek() {
                    value = value * 10 + (*n as usize - b'0' as usize);
                    chars.next();
                }
                Ok(Self::Primitive(value))
            }
        }
    }
}

impl Add for SFNum {
    type Output = SFNum;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = SFNum::Compound(Box::new(self), Box::new(rhs));
        result.reduce();
        result
    }
}

impl Sum for SFNum {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        if let Some(mut result) = iter.next() {
            for next in iter {
                result = result + next;
            }
            result
        } else {
            Self::Primitive(0)
        }
    }
}

impl FromStr for SFNum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut chars = s.chars().peekable();
        SFNum::parse(&mut chars)
    }
}

struct Day18;

impl Problem for Day18 {
    type Input = Vec<SFNum>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        input.iter().cloned().sum::<SFNum>().magnitude()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut max = 0;
        for (i, a) in input.iter().enumerate() {
            for (j, b) in input.iter().enumerate() {
                if i != j {
                    let sum = a.clone() + b.clone();
                    max = usize::max(max, sum.magnitude());
                }
            }
        }
        max
    }
}

fn main() {
    solve_main::<Day18>();
}
