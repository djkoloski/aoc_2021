use ::core::fmt;
use anyhow::{anyhow, Result};
use problem::{solve_main, Problem};

enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

impl Fold {
    fn transform_point(&self, point: (i32, i32)) -> (i32, i32) {
        match self {
            Fold::Horizontal(y) => (point.0, y - (point.1 - y).abs()),
            Fold::Vertical(x) => (x - (point.0 - x).abs(), point.1),
        }
    }
}

struct Input {
    points: Vec<(i32, i32)>,
    folds: Vec<Fold>,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut result = Input {
            points: Vec::new(),
            folds: Vec::new(),
        };

        let mut lines = reader.lines();
        for line in lines.by_ref() {
            let line = line?;
            if line.is_empty() {
                break;
            }

            let mut coords = line.split(',');
            let x = coords
                .next()
                .ok_or_else(|| anyhow!("Missing X coord"))?
                .parse()?;
            let y = coords
                .next()
                .ok_or_else(|| anyhow!("Missing Y coord"))?
                .parse()?;
            result.points.push((x, y));
        }

        for line in lines {
            let line = line?;
            if let Some(x) = line.strip_prefix("fold along x=") {
                result.folds.push(Fold::Vertical(x.parse()?));
            } else if let Some(y) = line.strip_prefix("fold along y=") {
                result.folds.push(Fold::Horizontal(y.parse()?));
            } else {
                return Err(anyhow!("Invalid fold instruction"));
            }
        }

        Ok(result)
    }
}

fn fold_points(points: &mut Vec<(i32, i32)>, fold: &Fold) {
    for point in points {
        *point = fold.transform_point(*point);
    }
}

struct Display {
    lines: Vec<Vec<bool>>,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for line in self.lines.iter() {
            for p in line {
                if *p {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Day13;

impl Problem for Day13 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = Display;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut points = input.points.clone();
        fold_points(&mut points, &input.folds[0]);
        points.sort_unstable();
        points.dedup();
        points.len()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut points = input.points.clone();
        for fold in input.folds.iter() {
            fold_points(&mut points, fold);
        }
        points.sort_unstable();
        points.dedup();

        let left = points.iter().map(|p| p.0).min().unwrap();
        let right = points.iter().map(|p| p.0).max().unwrap();
        let bottom = points.iter().map(|p| p.1).min().unwrap();
        let top = points.iter().map(|p| p.1).max().unwrap();

        let mut result = Display { lines: Vec::new() };
        for _ in bottom..=top {
            result.lines.push(vec![false; (right - left + 1) as usize]);
        }
        for (x, y) in points {
            result.lines[y as usize][x as usize] = true;
        }
        result
    }
}

fn main() {
    solve_main::<Day13>();
}
