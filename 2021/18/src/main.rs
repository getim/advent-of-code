use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;


#[derive(Default)]
struct ExplodeResult {
    did_explode: bool,
    left_increment: Option<u32>,
    right_increment: Option<u32>,
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

#[derive(Debug,Clone)]
enum Number {
    Zero,
    Regular(u32),
    Nested(Box<Number>, Box<Number>),
}

#[derive(Debug)]
struct NumberParseError;

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Zero => write!(f, "zero"),
            Number::Regular(n) => write!(f, "{}", n),
            Number::Nested(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

impl FromStr for Number {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str_iter = s.chars();
        // Drop leading '[' to immediately start reading the top-level number
        str_iter.next();
        number_from_char_iter(&mut str_iter)
    }
}

impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Number::Zero => other,
            _ => match other {
                Number::Zero => self,
                _ => {
                    let mut result = Number::Nested(Box::new(self), Box::new(other));
                    result.reduce();
                    result
                }
            }
        }
    }
}

impl Sum for Number {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item=Self> {
        iter.fold(Number::Zero, |accum, n| accum + n)
    }
}

impl Number {
    fn reduce(&mut self) {
        if self.try_explode(0).did_explode || self.try_split() {
            self.reduce();
        }
    }

    fn try_explode(&mut self, depth: usize) -> ExplodeResult {
        match self {
            Number::Zero => panic!("Aaaah, zero!"),
            Number::Regular(_) => ExplodeResult { ..Default::default() },
            Number::Nested(l, r) => {
                if depth == 4 {
                    if let Number::Regular(l_inner) = **l {
                        if let Number::Regular(r_inner) = **r {
                            *self = Number::Regular(0);
                            return ExplodeResult {
                                did_explode: true,
                                left_increment: Some(l_inner),
                                right_increment: Some(r_inner),
                            }
                        }
                    }
                    panic!("Tried exploding {}, that doesn't work", self);
                } else {
                    let mut l_result = l.try_explode(depth + 1);
                    if l_result.did_explode {
                        if let Some(r_increment) = l_result.right_increment {
                            if r.try_increment(r_increment, Side::Left) {
                                l_result.right_increment = None;
                            }
                        }
                        l_result
                    } else {
                        let mut r_result = r.try_explode(depth + 1);
                        if r_result.did_explode {
                            if let Some(l_increment) = r_result.left_increment {
                                if l.try_increment(l_increment, Side::Right) {
                                    r_result.left_increment = None;
                                }
                            }
                        }
                        r_result
                    }
                }
            }
        }
    }

    fn try_increment(&mut self, increment: u32, side: Side) -> bool {
        match self {
            Number::Zero => panic!("Aaaah, zero!"),
            Number::Regular(n) => {
                *self = Number::Regular(*n + increment);
                true
            }
            Number::Nested(l, r) => {
                match side {
                    Side::Left => l.try_increment(increment, side),
                    Side::Right => r.try_increment(increment, side),
                }
            }
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            Number::Zero => panic!("Aaaah, zero!"),
            Number::Regular(n) => {
                if *n >= 10 {
                    *self = Number::Nested(Box::new(Number::Regular(*n / 2)), Box::new(Number::Regular((*n + 1) / 2)));
                    true
                } else {
                    false
                }
            },
            Number::Nested(l, r) => l.try_split() || r.try_split(),
        }
    }

    fn magnitude(self) -> u32 {
        match self {
            Number::Zero => panic!("Aaaah, zero!"),
            Number::Regular(n) => n,
            Number::Nested(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let numbers: Vec<_> = BufReader::new(file).lines().map(|l| Number::from_str(&l.unwrap()).unwrap()).collect();

    println!("Result 1: {}", solve1(&numbers));
    println!("Result 2: {}", solve2(&numbers));
}

fn solve1(numbers: &[Number]) -> u32 {
    numbers.iter().cloned().sum::<Number>().magnitude()
}

fn solve2(numbers: &[Number]) -> u32 {
    let len = numbers.len();
    let mut highest_magnitude = 0;
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue
            }
            let magnitude = (numbers[i].clone() + numbers[j].clone()).magnitude();
            if magnitude > highest_magnitude {
                highest_magnitude = magnitude;
            }
        }
    }
    highest_magnitude
}

fn number_from_char_iter<I>(s: &mut I) -> Result<Number, NumberParseError>
    where I: Iterator<Item=char>
{
    let mut result = VecDeque::with_capacity(2);
    loop {
        let c = s.next().unwrap();
        match c {
            ',' => (),
            ']' => break,
            '[' => result.push_back(number_from_char_iter(s)?),
            c => match c.to_digit(10) {
                Some(d) => result.push_back(Number::Regular(d)),
                _ => return Err(NumberParseError),
            }
        }
    }
    if result.len() != 2 {
        Err(NumberParseError)
    } else {
        Ok(Number::Nested(Box::new(result.pop_front().unwrap()),
                          Box::new(result.pop_front().unwrap())))
    }
}
