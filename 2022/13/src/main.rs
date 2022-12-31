use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::Chars;


#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Int(usize),
}

impl Packet {
    fn from_line(line: &str) -> Self {
        Self::_from_tokens(&mut Self::_tokenize_chars(&mut line.chars()))
    }

    fn _tokenize_chars(chars: &mut Chars) -> VecDeque<Token> {
        let mut tokens = VecDeque::new();
        loop {
            let c = chars.next();
            match c {
                Some('[') => tokens.push_back(Token::Open),
                Some(']') => tokens.push_back(Token::Close),
                Some(',') => (),
                Some(mut d) => {
                    let mut digits: Vec<char> = vec![];
                    while d != ',' && d != ']' {
                        digits.push(d);
                        d = chars.next().unwrap();
                    };
                    tokens.push_back(Token::Int(
                        digits.iter().collect::<String>().parse::<usize>().unwrap()
                    ));
                    if d == ']' {
                        tokens.push_back(Token::Close)
                    };
                },
                None => break,
            }
        }
        tokens
    }

    fn _from_tokens(tokens: &mut VecDeque<Token>) -> Self {
        match tokens.pop_front() {
            Some(Token::Open) => {
                let mut elements = vec![];
                loop {
                    if tokens[0] == Token::Close {
                        tokens.pop_front();
                        break;
                    }
                    elements.push(Self::_from_tokens(tokens));
                }
                Packet::List(elements)
            }
            Some(Token::Int(d)) => Packet::Int(d),
            _ => panic!(),
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a Packet {
    type Item = <std::slice::Iter<'a, Packet> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, Packet>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Packet::List(l) => l.as_slice().iter(),
            Packet::Int(_) => [].iter(),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => {
                match l.cmp(r) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Ordering::Greater),
                }
            },
            (Packet::List(l), Packet::List(r)) => {
                let mut r_iter = r.iter();
                for next_l in l.iter() {
                    if let Some(next_r) = r_iter.next() {
                        if let Some(is_ordered) = next_l.partial_cmp(next_r) {
                            return Some(is_ordered)
                        } else {
                            continue
                        }
                    } else {
                        return Some(Ordering::Greater)
                    }
                }
                if r_iter.next().is_some() {
                    Some(Ordering::Less)
                } else {
                    None
                }
            },
            (Packet::List(l), Packet::Int(r)) => {
                Packet::List(l.to_vec()).partial_cmp(&Packet::List(vec![Packet::Int(*r)]))
            },
            (Packet::Int(l), Packet::List(r)) => {
                Packet::List(vec![Packet::Int(*l)]).partial_cmp(&Packet::List(r.to_vec()))
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug,PartialEq)]
enum Token {
    Open,
    Close,
    Int(usize),
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn is_ordered(&self) -> bool {
        self.left < self.right
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut pairs: Vec<Pair> = vec![];
    let mut packets: Vec<Packet> = vec![];
    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let left = Packet::from_line(&line.unwrap().unwrap());
        let right = Packet::from_line(&lines.next().unwrap().unwrap());
        pairs.push(Pair { left: left.clone(), right: right.clone() });
        packets.extend_from_slice(&[left, right]);
        lines.next();
    }

    packets.extend_from_slice(&[
        Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
    ]);

    println!("Result 1: {:?}", solve1(&pairs));
    println!("Result 2: {:?}", solve2(&mut packets));
}

fn solve1(pairs: &[Pair]) -> usize {
    let mut result = 0;
    for (i, pair) in pairs.iter().enumerate() {
        if pair.is_ordered() {
            result += i + 1;
        }
    }
    result
}

fn solve2(packets: &mut Vec<Packet>) -> usize {
    packets.sort();
    let first_pos = packets.iter().position(|p| *p == Packet::List(vec![Packet::List(vec![Packet::Int(2)])])).unwrap();
    let second_pos = packets.iter().position(|p| *p == Packet::List(vec![Packet::List(vec![Packet::Int(6)])])).unwrap();
    (first_pos + 1) * (second_pos + 1)
}
