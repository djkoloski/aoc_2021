use ::core::str::FromStr;
use std::collections::HashMap;
use anyhow::{anyhow, Error};
use problem::{solve_main, Problem};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',').map(|x| x.parse());
        Ok(Self {
            x: coords.next().ok_or(anyhow!("Missing x coordinate"))??,
            y: coords.next().ok_or(anyhow!("Missing y coordinate"))??,
        })
    }
}

struct Line(Point, Point);

impl Line {
    fn is_straight(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let x0 = self.0.x;
        let y0 = self.0.y;
        let dx = self.1.x - self.0.x;
        let dy = self.1.y - self.0.y;
        let n = gcd(dy, dx).abs();
        let sx = dx / n;
        let sy = dy / n;
        (0..=n).map(move |i| Point { x: x0 + sx * i, y: y0 + sy * i })
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ").map(|x| x.parse());
        Ok(Self(
            points.next().ok_or(anyhow!("Missing start point"))??,
            points.next().ok_or(anyhow!("Missing end point"))??,
        ))
    }
}

fn count_duplicate_points<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    let mut points = HashMap::new();
    for line in lines {
        for point in line.points() {
            *points.entry(point).or_insert(0) += 1;
        }
    }
    points.values().filter(|&x| *x > 1).count()
}

struct Day5;

impl Problem for Day5 {
    type Input = Vec<Line>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        count_duplicate_points(input.iter().filter(|l| l.is_straight()))
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        count_duplicate_points(input.iter())
    }
}

fn main() {
    solve_main::<Day5>();
}
