use anyhow::{anyhow, Context, Result};
use std::{
    env,
    fmt::{self, Display},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
    time::{Duration, Instant},
};

pub trait Input: Sized {
    fn parse<R: BufRead>(reader: R) -> Result<Self>;
}

impl<T: FromStr> Input for Vec<T>
where
    T::Err: Display,
{
    fn parse<R: BufRead>(reader: R) -> Result<Self> {
        reader
            .lines()
            .enumerate()
            .map(|(line_number, line)| {
                T::from_str(&line.context("Failed to read line")?)
                    .map_err(|e| anyhow!("Failed to parse line {}: {}", line_number + 1, e))
            })
            .collect()
    }
}

pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "unimplemented")
    }
}

pub trait Problem {
    type Input: Input;
    type PartOne: Display;
    type PartTwo: Display;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne;
    fn solve_part_two(input: &Self::Input) -> Self::PartTwo;
}

pub struct Solution<T> {
    result: T,
    duration: Duration,
}

impl<T: Display> Display for Solution<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  Solution: {}", self.result)?;
        writeln!(f, "  Elapsed:  {} seconds", self.duration.as_secs_f64())?;
        Ok(())
    }
}

fn time_solve<F: FnOnce() -> T, T>(f: F) -> Solution<T> {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now().duration_since(start);
    Solution { result, duration }
}

pub type SolveResult<P> = Result<(
    Solution<<P as Problem>::PartOne>,
    Solution<<P as Problem>::PartTwo>,
)>;

pub fn solve<P: Problem>(path: &Path) -> SolveResult<P> {
    let input_file = BufReader::new(File::open(path).context("Failed to open input file")?);
    let input = Input::parse(input_file).context("Failed to parse input")?;

    Ok((
        time_solve(|| P::solve_part_one(&input)),
        time_solve(|| P::solve_part_two(&input)),
    ))
}

pub fn solve_main<P: Problem>() {
    let path = env::args().nth(1).expect("missing input file path");
    let (part_one, part_two) = solve::<P>(path.as_ref()).expect("failed to solve problem");

    println!("Part one:");
    println!("{}", part_one);
    println!("Part two:");
    println!("{}", part_two);
}
