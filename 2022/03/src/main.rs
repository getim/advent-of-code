use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


type Item = char;

pub trait Priority {
    fn get_priority(self) -> u32;
}

impl Priority for Item {
    fn get_priority(self) -> u32 {
        let ord: u32 = self.into();
        if self.is_lowercase() {
            ord - 96
        } else if self.is_uppercase() {
            ord - 64 + 26
        } else {
            panic!()
        }
    }
}

type Compartment = HashSet<Item>;

type Rucksack = [Compartment; 2];

pub trait UniqueItems {
    fn get_unique_items(&self) -> HashSet<char>;
}

impl UniqueItems for Rucksack {
    fn get_unique_items(&self) -> HashSet<char> {
        self[0].union(&self[1]).copied().collect()
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let rucksacks: Vec<Rucksack> = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let compartment_size = line.len() / 2;
        [
            line[..compartment_size].chars().collect(),
            line[compartment_size..].chars().collect(),
        ]
    }).collect();

    println!("Result 1: {}", solve1(&rucksacks));
    println!("Result 2: {}", solve2(&rucksacks));
}

fn solve1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().map(|rucksack|
        rucksack[0].intersection(&rucksack[1]).next().unwrap().get_priority()
    ).sum()
}

fn solve2(rucksacks: &[Rucksack]) -> u32 {
    let mut result = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        result += rucksacks[i].get_unique_items()
            .intersection(&rucksacks[i + 1].get_unique_items())
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&rucksacks[i + 2].get_unique_items())
            .next().unwrap()
            .get_priority();
    }
    result
}
