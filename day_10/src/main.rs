use problem::{solve_main, Problem};

struct Day10;

impl Problem for Day10 {
    type Input = Vec<String>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut score = 0;
        for line in input {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' => {
                        if stack.pop() != Some('(') {
                            score += 3;
                            break;
                        }
                    }
                    ']' => {
                        if stack.pop() != Some('[') {
                            score += 57;
                            break;
                        }
                    }
                    '}' => {
                        if stack.pop() != Some('{') {
                            score += 1197;
                            break;
                        }
                    }
                    '>' => {
                        if stack.pop() != Some('<') {
                            score += 25137;
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
        score
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut scores = Vec::new();
        'line: for line in input {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' => {
                        if stack.pop() != Some('(') {
                            continue 'line;
                        }
                    }
                    ']' => {
                        if stack.pop() != Some('[') {
                            continue 'line;
                        }
                    }
                    '}' => {
                        if stack.pop() != Some('{') {
                            continue 'line;
                        }
                    }
                    '>' => {
                        if stack.pop() != Some('<') {
                            continue 'line;
                        }
                    }
                    _ => break,
                }
            }

            let mut score = 0;
            while let Some(c) = stack.pop() {
                score = score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => break,
                    };
            }
            scores.push(score);
        }
        scores.sort_unstable();
        scores[scores.len() / 2]
    }
}

fn main() {
    solve_main::<Day10>();
}
