use ::core::str::FromStr;
use anyhow::{anyhow, Error};
use problem::{solve_main, Problem};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split(' ');
        let direction = pieces.next().ok_or(anyhow!("Missing direction"))?;
        let amount = pieces.next().ok_or(anyhow!("Missing amount"))?.parse()?;
        Ok(match direction {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => return Err(anyhow!("Invalid direction '{}'", direction)),
        })
    }
}

struct Day2;

impl Problem for Day2 {
    type Input = Vec<Command>;
    type PartOne = i32;
    type PartTwo = i32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let (pos, depth) = input.iter().fold((0, 0), |(pos, depth), command| match command {
            Command::Forward(amount) => (pos + amount, depth),
            Command::Down(amount) => (pos, depth + amount),
            Command::Up(amount) => (pos, depth - amount),
        });
        pos * depth
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let (pos, depth, _) = input.iter().fold((0, 0, 0), |(pos, depth, aim), command| match command {
            Command::Forward(amount) => (pos + amount, depth + aim * amount, aim),
            Command::Down(amount) => (pos, depth, aim + amount),
            Command::Up(amount) => (pos, depth, aim - amount),
        });
        pos * depth
    }
}

fn main() {
    solve_main::<Day2>();
}
