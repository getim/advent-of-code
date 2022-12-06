use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug)]
struct Move {
    crate_count: usize,
    src_id: usize,
    dest_id: usize,
}

impl Move {
    fn execute_9000(&self, stacks: &mut Vec<Stack>) {
        for _ in 0..self.crate_count {
            let moving_crate = stacks[self.src_id].pop().unwrap();
            stacks[self.dest_id].push(moving_crate);
        }
    }

    fn execute_9001(&self, stacks: &mut Vec<Stack>) {
        let src_stack = &mut stacks[self.src_id];
        let mut moving_crates = src_stack.split_off(src_stack.len() - self.crate_count);
        stacks[self.dest_id].append(&mut moving_crates);
    }

    fn from_line(line: &str) -> Move {
        let mut tokens = line.split_whitespace();
        Move {
            crate_count: tokens.nth(1).unwrap().parse().unwrap(),
            src_id: tokens.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            dest_id: tokens.nth(1).unwrap().parse::<usize>().unwrap() - 1,
        }
    }
}


fn main() {
    let file = File::open("input").unwrap();

    let mut stacks: Vec<Stack> = vec![];
    let mut moves: Vec<Move> = vec![];
    let mut is_reading_stacks = true;

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();

        if is_reading_stacks {
            if line.is_empty() {
                is_reading_stacks = false;
                continue;
            }
            read_stack_line(&mut stacks, &line);
        } else {
            moves.push(Move::from_line(&line));
        }
    }

    reverse_stacks(&mut stacks);

    println!("Result 1: {}", solve1(&mut stacks.clone(), &moves));
    println!("Result 1: {}", solve2(&mut stacks, &moves));
}

fn solve2(stacks: &mut Vec<Stack>, moves: &[Move]) -> String {
    for m in moves {
        m.execute_9001(stacks);
    }
    get_top_crates(stacks)
}

fn solve1(stacks: &mut Vec<Stack>, moves: &[Move]) -> String {
    for m in moves {
        m.execute_9000(stacks);
    }
    get_top_crates(stacks)
}

fn get_top_crates(stacks: &[Stack]) -> String {
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn reverse_stacks(stacks: &mut Vec<Stack>) {
    for stack in stacks {
        stack.reverse();
    }
}

fn read_stack_line(stacks: &mut Vec<Stack>, line: &str) {
    if stacks.is_empty() {
        for _ in 0..((line.chars().count() + 1) / 4) {
            stacks.push(vec![]);
        }
    }

    let mut chars = line.chars().peekable();
    chars.next();  // first column is not relevant
    let mut stack_id = 0;
    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        if c.is_alphabetic() {
            stacks[stack_id].push(c);
        }
        chars.nth(2);  // 3 columns contain decoration
        stack_id += 1;
    }
}
