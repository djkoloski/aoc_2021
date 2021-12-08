use problem::{solve_main, Problem, CSV};

struct Day7;

impl Problem for Day7 {
    type Input = CSV<i32>;
    type PartOne = i32;
    type PartTwo = i32;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut sorted = input.values().to_owned();
        sorted.sort_unstable();
        let median = sorted[sorted.len() / 2];
        sorted.iter().map(|&x| (median - x).abs()).sum::<i32>()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut current = *input.values().iter().min().unwrap();
        let max = *input.values().iter().max().unwrap();
        let mut cost = input
            .values()
            .iter()
            .map(|&x| {
                let dx = (x - current).abs();
                dx * (dx + 1) / 2
            })
            .sum();
        while current < max {
            let next = current + 1;
            let dc: i32 = input
                .values()
                .iter()
                .map(|&x| {
                    if current >= x {
                        current - x + 1
                    } else {
                        current - x
                    }
                })
                .sum();
            if dc > 0 {
                break;
            }
            cost += dc;
            current = next;
        }
        cost
    }
}

fn main() {
    solve_main::<Day7>();
}
