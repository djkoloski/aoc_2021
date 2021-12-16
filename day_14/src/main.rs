use ::core::str::FromStr;
use ::std::collections::HashMap;
use anyhow::{Error, Result};
use problem::{solve_main, Problem};

struct Rule {
    left: char,
    right: char,
    middle: char,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            left: s.chars().next().unwrap(),
            right: s.chars().nth(1).unwrap(),
            middle: s.chars().nth(6).unwrap(),
        })
    }
}

struct Input {
    template: String,
    rules: Vec<Rule>,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines();

        let template = lines.next().unwrap()?;
        lines.next();

        let rules = lines
            .map(|l| l?.parse::<Rule>())
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { template, rules })
    }
}

fn count_after(input: &Input, c: char, steps: usize) -> usize {
    let rules = input
        .rules
        .iter()
        .map(|r| ((r.left, r.right), r.middle))
        .collect();
    let mut cache = HashMap::new();
    let mut total = 0;
    for (left, right) in input.template.chars().zip(input.template.chars().skip(1)) {
        total += count_after_recurse(left, right, c, steps, &rules, &mut cache);
    }
    if input.template.chars().last().unwrap() == c {
        total += 1;
    }
    total
}

fn count_after_recurse(
    left: char,
    right: char,
    c: char,
    steps: usize,
    rules: &HashMap<(char, char), char>,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    if steps == 0 {
        if left == c {
            1
        } else {
            0
        }
    } else if let Some(result) = cache.get(&(left, right, steps)) {
        *result
    } else if let Some(&middle) = rules.get(&(left, right)) {
        let result = count_after_recurse(left, middle, c, steps - 1, rules, cache)
            + count_after_recurse(middle, right, c, steps - 1, rules, cache);
        cache.insert((left, right, steps), result);
        result
    } else if left == c {
        1
    } else {
        0
    }
}

struct Day14;

impl Problem for Day14 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut chars = Vec::new();
        for rule in input.rules.iter() {
            chars.push(rule.left);
            chars.push(rule.right);
            chars.push(rule.middle);
        }
        chars.sort_unstable();
        chars.dedup();

        let counts = chars
            .iter()
            .map(|&c| count_after(input, c, 10))
            .collect::<Vec<_>>();
        counts.iter().max().unwrap() - counts.iter().min().unwrap()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut chars = Vec::new();
        for rule in input.rules.iter() {
            chars.push(rule.left);
            chars.push(rule.right);
            chars.push(rule.middle);
        }
        chars.sort_unstable();
        chars.dedup();

        let counts = chars
            .iter()
            .map(|&c| count_after(input, c, 40))
            .collect::<Vec<_>>();
        counts.iter().max().unwrap() - counts.iter().min().unwrap()
    }
}

fn main() {
    solve_main::<Day14>();
}
