use problem::{solve_main, Problem};
use anyhow::{anyhow, Result};

#[derive(Default)]
struct Board {
    numbers: [u8; 25],
}

impl Board {
    fn earliest_win(&self, order: &[u8; 100]) -> u8 {
        let mut earliest = 100;
        for i in 0..5 {
            let mut latest = 0;
            for j in 0..5 {
                latest = u8::max(latest, order[self.numbers[5 * i + j] as usize]);
            }
            earliest = u8::min(earliest, latest);
        }
        for j in 0..5 {
            let mut latest = 0;
            for i in 0..5 {
                latest = u8::max(latest, order[self.numbers[5 * i + j] as usize]);
            }
            earliest = u8::min(earliest, latest);
        }
        earliest
    }

    fn unmarked_total(&self, order: &[u8; 100], draw: u8) -> usize {
        let mut unmarked = 0;
        for i in self.numbers {
            if order[i as usize] > draw {
                unmarked += i as usize;
            }
        }
        unmarked
    }
}

struct Input {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl problem::Input for Input {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines();

        let numbers = lines.next().ok_or(anyhow!("Misisng numbers line"))??.split(',').map(|x| Ok(x.parse::<u8>()?)).collect::<Result<Vec<_>>>()?;

        let mut boards = Vec::new();
        while let Some(blank) = lines.next() {
            let mut line = blank?;
            if line != "" {
                let mut board = Board::default();
                for i in 0..5 {
                    for (j, n) in line.split(' ').filter(|&n| n != "").map(|n| n.parse()).enumerate() {
                        board.numbers[5 * i + j] = n?;
                    }
                    if i < 4 {
                        line = lines.next().ok_or(anyhow!("Missing line {} of a board", i))??;
                    }
                }
                boards.push(board);
            }
        }

        Ok(Self {
            numbers,
            boards,
        })
    }
}

struct Day4;

impl Problem for Day4 {
    type Input = Input;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut order = [0; 100];
        for (i, &n) in input.numbers.iter().enumerate() {
            order[n as usize] = i as u8;
        }

        let winner = input.boards.iter().min_by_key(|&b| b.earliest_win(&order)).unwrap();
        let draw = winner.earliest_win(&order);
        let total = winner.unmarked_total(&order, draw);
        let last = input.numbers[draw as usize] as usize;
        total * last
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut order = [0; 100];
        for (i, &n) in input.numbers.iter().enumerate() {
            order[n as usize] = i as u8;
        }

        let loser = input.boards.iter().max_by_key(|&b| b.earliest_win(&order)).unwrap();
        let draw = loser.earliest_win(&order);
        let total = loser.unmarked_total(&order, draw);
        let last = input.numbers[draw as usize] as usize;
        total * last
    }
}

fn main() {
    solve_main::<Day4>();
}
