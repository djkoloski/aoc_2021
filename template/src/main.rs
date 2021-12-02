use problem::{solve_main, Problem};

struct DayN;

impl Problem for DayN {
    type Input = Vec<i32>;
    type PartOne = problem::Unimplemented;
    type PartTwo = problem::Unimplemented;

    fn solve_part_one(_input: &Self::Input) -> Self::PartOne {
        problem::Unimplemented
    }

    fn solve_part_two(_input: &Self::Input) -> Self::PartTwo {
        problem::Unimplemented
    }
}

fn main() {
    solve_main::<DayN>();
}
