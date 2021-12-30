use anyhow::{anyhow, Result};
use problem::{solve_main, Problem};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::Range,
};

// Board layout:
// #############
// #01.2.3.4.56#
// ###8#9#A#B###
//   #C#D#E#F#
//   ...
//   #########
// 7 means an amphipod is in its home and can no longer move

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Position<const N: usize>(pub u8);

impl<const N: usize> Position<N> {
    #[inline]
    pub fn is_done(self) -> bool {
        self.0 == 7
    }

    #[inline]
    pub fn is_room(self) -> bool {
        self.0 > 7
    }

    #[inline]
    pub fn is_hallway(self) -> bool {
        self.0 <= 6
    }

    #[inline]
    pub fn room_x(self) -> usize {
        (self.0 & 0x3) as usize
    }

    #[inline]
    pub fn room_y(self) -> usize {
        ((self.0 - 8) / 4) as usize
    }
}

const POS_DONE: u8 = 0x7;

fn blocker_range(r: usize, h: usize) -> Range<u8> {
    u8::min(2 + r as u8, h as u8 + 1)..u8::max(2 + r as u8, h as u8)
}

const DIST_HALLWAY_TO_ROOM: [[u8; 4]; 7] = [
    [3, 5, 7, 9],
    [2, 4, 6, 8],
    [2, 2, 4, 6],
    [4, 2, 2, 4],
    [6, 4, 2, 2],
    [8, 6, 4, 2],
    [9, 7, 5, 3],
];

#[derive(Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct State<const N: usize> {
    amphipods: [[Position<N>; N]; 4],
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        Self {
            amphipods: [[Position(POS_DONE); N]; 4],
        }
    }
}

pub struct Transition<const N: usize> {
    t: usize,
    n: usize,
    destination: Position<N>,
    cost: usize,
}

impl<const N: usize> State<N> {
    pub const POS_DONE: Position<N> = Position(POS_DONE);

    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn get(&self, t: usize, n: usize) -> Position<N> {
        self.amphipods[t][n]
    }

    #[inline]
    pub fn set(&mut self, t: usize, n: usize, position: Position<N>) {
        self.amphipods[t][n] = position;
    }

    #[inline]
    pub fn positions(&self) -> impl Iterator<Item = Position<N>> + '_ {
        self.amphipods.into_iter().flatten()
    }

    #[inline]
    pub fn reduce(&mut self) {
        for i in 0..4 {
            self.amphipods[i].sort_unstable();
        }
    }

    #[inline]
    pub fn initialize(&mut self) {
        for _ in 0..N {
            for n in 0..N {
                for t in 0..4 {
                    let pos = self.get(t, n);
                    if pos.room_x() == t && pos.room_y() == N - 1 - self.finished_of_type(t) {
                        self.set(t, n, State::POS_DONE);
                    }
                }
            }
        }
        self.reduce();
    }

    #[inline]
    pub fn is_room_open(&self, room: usize) -> bool {
        self.positions()
            .all(|pos| !pos.is_room() || pos.room_x() != room)
    }

    #[inline]
    pub fn is_path_clear(&self, room: usize, hallway: usize) -> bool {
        blocker_range(room, hallway)
            .all(|blocker| self.positions().all(|pos| pos != Position(blocker)))
    }

    #[inline]
    pub fn can_exit(&self, pos: Position<N>) -> bool {
        self.positions()
            .all(|p| p.room_x() != pos.room_x() || p.room_y() >= pos.room_y())
    }

    #[inline]
    pub fn finished_of_type(&self, t: usize) -> usize {
        self.amphipods[t].iter().filter(|p| p.is_done()).count()
    }

    pub fn transitions(&self) -> Vec<Transition<N>> {
        const COST: [usize; 4] = [1, 10, 100, 1000];
        let mut transitions = Vec::new();
        for n in 0..N {
            for (t, cost) in COST.iter().enumerate() {
                let pos = self.get(t, n);
                if pos != Self::POS_DONE {
                    if pos.is_hallway() {
                        if self.is_room_open(t) && self.is_path_clear(t, pos.0 as usize) {
                            let distance = DIST_HALLWAY_TO_ROOM[pos.0 as usize][t] as usize + N
                                - 1
                                - self.finished_of_type(t);
                            transitions.push(Transition {
                                t,
                                n,
                                destination: Self::POS_DONE,
                                cost: distance * cost,
                            });
                        }
                    } else if self.can_exit(pos) {
                        for h in 0..7 {
                            let room_x = pos.room_x();
                            if self.positions().all(|p| p != Position(h))
                                && self.is_path_clear(room_x, h as usize)
                            {
                                let distance = DIST_HALLWAY_TO_ROOM[h as usize][room_x] as usize
                                    + pos.room_y();
                                transitions.push(Transition {
                                    t,
                                    n,
                                    destination: Position(h),
                                    cost: distance * COST[t],
                                });
                            }
                        }
                    }
                }
            }
        }
        transitions
    }

    pub fn solve(&self) -> usize {
        let mut visited = HashSet::new();
        let mut frontier = BinaryHeap::new();

        frontier.push((Reverse(0), self.clone()));

        let mut result = None;
        'outer: while let Some((Reverse(cost), state)) = frontier.pop() {
            if !visited.contains(&state) {
                if state.is_solved() {
                    result = Some(cost);
                    break 'outer;
                }

                for transition in state.transitions() {
                    let next_cost = cost + transition.cost;
                    let mut next = state.clone();
                    next.transition(transition);
                    next.reduce();

                    frontier.push((Reverse(next_cost), next));
                }

                visited.insert(state);
            }
        }

        result.unwrap()
    }

    #[inline]
    pub fn transition(&mut self, transition: Transition<N>) {
        self.set(transition.t, transition.n, transition.destination);
        self.reduce();
    }

    #[inline]
    fn is_solved(&self) -> bool {
        self.positions().all(|p| p == Self::POS_DONE)
    }

    #[inline]
    fn get_at(&self, pos: u8) -> Option<usize> {
        for t in 0..4 {
            for n in 0..N {
                if self.get(t, n).0 == pos {
                    return Some(t);
                }
            }
        }
        None
    }

    #[inline]
    fn char_at(&self, pos: u8) -> char {
        match self.get_at(pos) {
            None => '.',
            Some(0) => 'A',
            Some(1) => 'B',
            Some(2) => 'C',
            Some(3) => 'D',
            _ => unreachable!(),
        }
    }

    pub fn print(&self) {
        println!("#############");
        println!(
            "#{}{}.{}.{}.{}.{}{}#",
            self.char_at(0),
            self.char_at(1),
            self.char_at(2),
            self.char_at(3),
            self.char_at(4),
            self.char_at(5),
            self.char_at(6),
        );
        for n in 0..N {
            println!(
                "###{}#{}#{}#{}###",
                self.char_at(4 * n as u8 + 8),
                self.char_at(4 * n as u8 + 9),
                self.char_at(4 * n as u8 + 10),
                self.char_at(4 * n as u8 + 11),
            );
        }
        println!("  #########");
    }
}

impl<const N: usize> problem::Input for State<N> {
    fn parse<R: std::io::BufRead>(reader: R) -> Result<Self> {
        let mut lines = reader.lines();

        lines.next();
        lines.next();

        let mut state = State::new();
        let mut counts = [0; 4];
        for n in 0..N {
            let line = lines.next().ok_or(anyhow!("Missing line"))??;
            for (in_t, char_index) in [3, 5, 7, 9].into_iter().enumerate() {
                let t = match line
                    .chars()
                    .nth(char_index)
                    .ok_or(anyhow!("Invalid line length"))?
                {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    _ => return Err(anyhow!("Unrecognized amphipod type")),
                };
                state.amphipods[t][counts[t]] = Position((8 + 4 * n + in_t) as u8);
                counts[t] += 1;
            }
        }

        Ok(state)
    }
}

struct Day23;

impl Problem for Day23 {
    type Input = State<2>;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        let mut state = input.clone();
        state.initialize();
        state.solve()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        let mut state = State::<4>::new();
        for t in 0..4 {
            for n in 0..2 {
                let pos = input.get(t, n);
                let mut new_pos = Position(pos.0);
                if new_pos.0 >= 12 {
                    new_pos.0 += 8;
                }
                state.set(t, n, new_pos);
            }
        }
        state.set(0, 2, Position(15));
        state.set(0, 3, Position(18));
        state.set(1, 2, Position(14));
        state.set(1, 3, Position(17));
        state.set(2, 2, Position(13));
        state.set(2, 3, Position(19));
        state.set(3, 2, Position(12));
        state.set(3, 3, Position(16));

        state.initialize();
        state.solve()
    }
}

fn main() {
    solve_main::<Day23>();
}
