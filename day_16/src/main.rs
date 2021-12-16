use ::core::iter::{ExactSizeIterator, Iterator};
use anyhow::Result;
use problem::{solve_main, Problem};

struct Bits<'a> {
    bytes: &'a [u8],
    index: usize,
    bit: u8,
}

impl<'a> Iterator for Bits<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit == 8 {
            self.bit = 0;
            self.index += 1;
        }

        if self.index < self.bytes.len() {
            let result = self.bytes[self.index] & (1 << self.bit);
            self.bit += 1;
            Some(result != 0)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for Bits<'a> {
    fn len(&self) -> usize {
        (self.bytes.len() - self.index) * 8 - self.bit as usize
    }
}

fn int_from_bits<I: Iterator<Item = bool>, const N: usize>(bits: &mut I) -> usize {
    let mut result = 0;
    for _ in 0..N {
        result <<= 1;
        if bits.next().unwrap() {
            result |= 1;
        }
    }
    result
}

fn varint_from_bits<I: Iterator<Item = bool>>(bits: &mut I) -> usize {
    let mut result = 0;
    loop {
        let end = !bits.next().unwrap();
        result <<= 4;
        result |= int_from_bits::<_, 4>(bits);
        if end {
            break;
        }
    }
    result
}

#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum Payload {
    Operator {
        operation: Operation,
        packets: Vec<Packet>,
    },
    Literal(usize),
}

impl Payload {
    fn from_bits<I: ExactSizeIterator<Item = bool>>(bits: &mut I) -> Self {
        let packet_type = int_from_bits::<_, 3>(bits);
        if packet_type == 4 {
            Payload::Literal(varint_from_bits(bits))
        } else {
            let mut packets = Vec::new();
            if !bits.next().unwrap() {
                // 15-bit length-encoded
                let length = int_from_bits::<_, 15>(bits);
                let initial_len = bits.len();
                while initial_len - bits.len() < length {
                    packets.push(Packet::from_bits(bits));
                }
            } else {
                // 11-bit number of sub-packets
                let length = int_from_bits::<_, 11>(bits);
                for _ in 0..length {
                    packets.push(Packet::from_bits(bits));
                }
            }
            Payload::Operator {
                operation: match packet_type {
                    0 => Operation::Sum,
                    1 => Operation::Product,
                    2 => Operation::Minimum,
                    3 => Operation::Maximum,
                    5 => Operation::GreaterThan,
                    6 => Operation::LessThan,
                    7 => Operation::EqualTo,
                    _ => unreachable!(),
                },
                packets,
            }
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Payload::Operator {
                operation: _,
                packets,
            } => packets.iter().map(|p| p.version_sum()).sum(),
            Payload::Literal(_) => 0,
        }
    }

    fn evaluate(&self) -> usize {
        match self {
            Payload::Operator { operation, packets } => {
                let mut values = packets.iter().map(|p| p.evaluate());
                match *operation {
                    Operation::Sum => values.sum(),
                    Operation::Product => values.product(),
                    Operation::Minimum => values.min().unwrap(),
                    Operation::Maximum => values.max().unwrap(),
                    _ => {
                        let lhs = values.next().unwrap();
                        let rhs = values.next().unwrap();
                        match *operation {
                            Operation::GreaterThan => {
                                if lhs > rhs {
                                    1
                                } else {
                                    0
                                }
                            }
                            Operation::LessThan => {
                                if lhs < rhs {
                                    1
                                } else {
                                    0
                                }
                            }
                            Operation::EqualTo => {
                                if lhs == rhs {
                                    1
                                } else {
                                    0
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
            Payload::Literal(value) => *value,
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    payload: Payload,
}

impl Packet {
    fn from_bits<I: ExactSizeIterator<Item = bool>>(bits: &mut I) -> Self {
        let version = int_from_bits::<_, 3>(bits);
        Self {
            version,
            payload: Payload::from_bits(bits),
        }
    }

    fn version_sum(&self) -> usize {
        self.version + self.payload.version_sum()
    }

    fn evaluate(&self) -> usize {
        self.payload.evaluate()
    }
}

fn from_hex(a: u8, b: u8) -> u8 {
    fn h2b(x: u8) -> u8 {
        if (b'0'..b'9').contains(&x) {
            x - b'0'
        } else {
            x - b'A' + 10
        }
    }
    h2b(a).reverse_bits() >> 4 | h2b(b).reverse_bits()
}

impl problem::Input for Packet {
    fn parse<R: std::io::BufRead>(mut reader: R) -> Result<Self> {
        let mut bytes = Vec::new();
        while !reader.fill_buf()?.is_empty() {
            let chars = reader.fill_buf()?;
            let mut consumed = 0;
            for (a, b) in chars.iter().step_by(2).zip(chars.iter().skip(1).step_by(2)) {
                bytes.push(from_hex(*a, *b));
                consumed += 2;
            }
            reader.consume(consumed);
        }

        let mut bits = Bits {
            bytes: &bytes,
            index: 0,
            bit: 0,
        };
        Ok(Packet::from_bits(&mut bits))
    }
}

struct Day16;

impl Problem for Day16 {
    type Input = Packet;
    type PartOne = usize;
    type PartTwo = usize;

    fn solve_part_one(input: &Self::Input) -> usize {
        input.version_sum()
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        input.evaluate()
    }
}

fn main() {
    solve_main::<Day16>();
}
