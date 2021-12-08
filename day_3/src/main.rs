use anyhow::{anyhow, Result};
use problem::{solve_main, Problem};

type Entry = u16;

#[derive(Debug)]
struct Input {
    width: usize,
    entries: Vec<Entry>,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut width = 0;
        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line?;
            width = line.len();
            let mut entry: Entry = 0;
            for c in line.chars() {
                let b = match c {
                    '0' => 0,
                    '1' => 1,
                    _ => return Err(anyhow!("Invalid char: '{}'", c)),
                };
                entry = entry << 1 | b;
            }
            entries.push(entry);
        }
        Ok(Self { width, entries })
    }
}

fn find_rating(input: &Input, value: Entry) -> usize {
    let mut candidates = input.entries.clone();
    let mut i = input.width;
    while candidates.len() > 1 {
        i -= 1;
        let ones = candidates
            .iter()
            .map(|&c| (c >> i & 1) as usize)
            .sum::<usize>();
        let zeros = candidates.len() - ones;
        let filter = if ones >= zeros { value } else { 1 - value };
        candidates.retain(|&c| c >> i & 1 == filter);
    }
    candidates[0] as usize
}

struct Day3;

impl Problem for Day3 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut gamma = 0;
        for i in (0..input.width).rev() {
            gamma <<= 1;
            let ones = input
                .entries
                .iter()
                .map(|&e| (e >> i & 1) as usize)
                .sum::<usize>();
            if ones >= input.entries.len() - ones {
                gamma |= 1;
            }
        }
        let epsilon = gamma ^ ((1 << input.width) - 1);
        gamma * epsilon
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let oxygen = find_rating(input, 1);
        let co2 = find_rating(input, 0);
        oxygen * co2
    }
}

fn main() {
    solve_main::<Day3>();
}
