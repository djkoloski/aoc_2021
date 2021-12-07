use problem::{solve_main, CSV, Problem};

fn simulate(initial: &[usize], duration: usize) -> u64 {
    let mut count = [0; 9];
    for i in initial.iter() {
        count[*i] += 1;
    }

    for _ in 0..duration {
        let zeros = count[0];
        for i in 0..8 {
            count[i] = count[i + 1];
        }
        count[8] = zeros;
        count[6] += zeros;
    }

    count.iter().sum()
}

struct Day6;

impl Problem for Day6 {
    type Input = CSV<usize>;
    type PartOne = u64;
    type PartTwo = u64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        simulate(input.values(), 80)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        simulate(input.values(), 256)
    }
}

fn main() {
    solve_main::<Day6>();
}
