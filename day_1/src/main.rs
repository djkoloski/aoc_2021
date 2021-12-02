use problem::{solve_main, Problem};

struct Day1;

impl Problem for Day1 {
    type Input = Vec<i32>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        input
            .iter()
            .zip(input.iter().skip(1))
            .filter(|(prev, next)| next > prev)
            .count()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        input
            .iter()
            .zip(input.iter().skip(3))
            .filter(|(prev, next)| next > prev)
            .count()
    }
}

fn main() {
    solve_main::<Day1>();
}
