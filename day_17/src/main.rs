use anyhow::{anyhow, Result};
use problem::{solve_main, Problem};

struct TargetArea {
    left: i32,
    right: i32,
    bottom: i32,
    top: i32,
}

impl problem::Input for TargetArea {
    fn parse<R: std::io::BufRead>(mut reader: R) -> Result<Self> {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut pieces = line
            .strip_prefix("target area: ")
            .ok_or(anyhow!("Invalid target area format"))?
            .split(", ");
        let x = pieces.next().ok_or(anyhow!("Missing x area"))?;
        let mut x_range = x
            .strip_prefix("x=")
            .ok_or(anyhow!("Invalid x area format"))?
            .split("..")
            .map(|s| s.parse());
        let y = pieces.next().ok_or(anyhow!("missing y area"))?;
        let mut y_range = y
            .strip_prefix("y=")
            .ok_or(anyhow!("Inavlid y area format"))?
            .split("..")
            .map(|s| s.parse());
        Ok(Self {
            left: x_range.next().ok_or(anyhow!("Missing left bound"))??,
            right: x_range.next().ok_or(anyhow!("Missing right bound"))??,
            bottom: y_range.next().ok_or(anyhow!("Missing bottom bound"))??,
            top: y_range.next().ok_or(anyhow!("Missing top bound"))??,
        })
    }
}

fn collect_initial_velocities(x: i32, y: i32, out: &mut Vec<(i32, i32)>) {
    let k = (1. + 8. * x as f64).sqrt() as i32;
    let is_stable = k * k == 8 * x + 1 && k % 2 == 1;
    let stable_steps = (k - 1) / 2;
    for s in 1..=stable_steps {
        let xi = (x + (s * (s - 1)) / 2) / s;
        let yi = (y + (s * (s - 1)) / 2) / s;
        if s * xi - s * (s - 1) / 2 == x && s * yi - s * (s - 1) / 2 == y {
            out.push((xi, yi));
        }
    }
    if is_stable {
        for s in stable_steps + 1..=-2 * y {
            let yi = (y + (s * (s - 1)) / 2) / s;
            if s * yi - s * (s - 1) / 2 == y {
                out.push((stable_steps, yi));
            }
        }
    }
}

struct Day17;

impl Problem for Day17 {
    type Input = TargetArea;
    type PartOne = i32;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        input.bottom * (input.bottom + 1) / 2
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut initial_velocities = Vec::new();
        for x in input.left..=input.right {
            for y in input.bottom..=input.top {
                collect_initial_velocities(x, y, &mut initial_velocities);
            }
        }
        initial_velocities.sort_unstable();
        initial_velocities.dedup();
        initial_velocities.len()
    }
}

fn main() {
    solve_main::<Day17>();
}
