use std::collections::HashMap;

use anyhow::{anyhow, Result};
use problem::{solve_main, Problem};

pub struct Input {
    p1_start: usize,
    p2_start: usize,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines();
        let p1_start = lines
            .next()
            .ok_or(anyhow!("Missing player 1 starting position"))??
            .strip_prefix("Player 1 starting position: ")
            .ok_or(anyhow!("Invalid player 1 starting position"))?
            .parse()?;
        let p2_start = lines
            .next()
            .ok_or(anyhow!("Missing player 2 starting position"))??
            .strip_prefix("Player 2 starting position: ")
            .ok_or(anyhow!("Invalid player 2 starting position"))?
            .parse()?;

        Ok(Self { p1_start, p2_start })
    }
}

struct Day21;

impl Problem for Day21 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut p1_score = 0;
        let mut p2_score = 0;
        let mut p1_pos = input.p1_start - 1;
        let mut p2_pos = input.p2_start - 1;
        let mut rolls = 0;

        fn roll(rolls: usize) -> usize {
            let r1 = rolls % 100;
            let r2 = (rolls + 1) % 100;
            let r3 = (rolls + 2) % 100;
            r1 + r2 + r3 + 3
        }

        let losing_score = loop {
            p1_pos = (p1_pos + roll(rolls)) % 10;
            rolls += 3;
            p1_score += p1_pos + 1;

            if p1_score >= 1000 {
                break p2_score;
            }

            p2_pos = (p2_pos + roll(rolls)) % 10;
            rolls += 3;
            p2_score += p2_pos + 1;

            if p2_score >= 1000 {
                break p1_score;
            }
        };

        losing_score * rolls
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        use ::core::ops::{Add, Mul};

        #[derive(Clone, Copy)]
        struct Score(usize, usize);

        impl Score {
            fn transpose(self) -> Self {
                Self(self.1, self.0)
            }
        }

        impl Add for Score {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Score(self.0 + rhs.0, self.1 + rhs.1)
            }
        }

        impl Mul<usize> for Score {
            type Output = Self;

            fn mul(self, rhs: usize) -> Self::Output {
                Score(rhs * self.0, rhs * self.1)
            }
        }

        #[inline]
        fn simulate(
            s1: usize,
            p1: usize,
            s2: usize,
            p2: usize,
            r: usize,
            cache: &mut HashMap<(usize, usize, usize, usize), Score>,
        ) -> Score {
            let new_pos = (p1 + r) % 10;
            let new_score = s1 + new_pos + 1;
            count_wins(s2, p2, new_score, new_pos, cache).transpose()
        }

        #[inline]
        fn count_wins(
            s1: usize,
            p1: usize,
            s2: usize,
            p2: usize,
            cache: &mut HashMap<(usize, usize, usize, usize), Score>,
        ) -> Score {
            if let Some(result) = cache.get(&(s1, p1, s2, p2)) {
                *result
            } else if s1 >= 21 {
                Score(1, 0)
            } else if s2 >= 21 {
                Score(0, 1)
            } else {
                // p1 rolls:
                // - 3 in 1/27 cases
                // - 4 in 3/27 cases
                // - 5 in 6/27 cases
                // - 6 in 7/27 cases
                // - 7 in 6/27 cases
                // - 8 in 3/27 cases
                // - 9 in 1/27 cases
                let result = simulate(s1, p1, s2, p2, 3, cache)
                    + simulate(s1, p1, s2, p2, 4, cache) * 3
                    + simulate(s1, p1, s2, p2, 5, cache) * 6
                    + simulate(s1, p1, s2, p2, 6, cache) * 7
                    + simulate(s1, p1, s2, p2, 7, cache) * 6
                    + simulate(s1, p1, s2, p2, 8, cache) * 3
                    + simulate(s1, p1, s2, p2, 9, cache);
                cache.insert((s1, p1, s2, p2), result);
                result
            }
        }

        let mut cache = HashMap::new();
        let Score(p1_wins, p2_wins) =
            count_wins(0, input.p1_start - 1, 0, input.p2_start - 1, &mut cache);
        usize::max(p1_wins, p2_wins)
    }
}

fn main() {
    solve_main::<Day21>();
}
