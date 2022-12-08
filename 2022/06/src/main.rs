use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let file = File::open("input").unwrap();
    let signal = BufReader::new(file).lines().next().unwrap().unwrap();

    println!("Result 1: {}", solve1(&signal, 4));
    println!("Result 2: {}", solve1(&signal, 14));
}

fn solve1(signal: &str, count: usize) -> usize {
    let mut uniques = VecDeque::new();

    for (i, c) in signal.chars().enumerate() {
        if let Some(p) = uniques.iter().position(|u| u == &c) {
            uniques = uniques.split_off(p + 1);
        }
        uniques.push_back(c);
        if uniques.len() == count {
            return i + 1
        }
    }

    panic!();
}
