use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Elf = Vec<u32>;

fn main() {
    let file = File::open("input").unwrap();
    let mut elf = vec![];
    let mut elves = vec![];

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        if line == "" {
            elves.push(elf);
            elf = vec![];
        } else {
            elf.push(line.parse::<u32>().unwrap());
        }
    }
    elves.push(elf);

    println!("Result 1: {:?}", solve1(&elves));
    println!("Result 2: {:?}", solve2(&elves));
}

fn solve1(elves: &[Elf]) -> u32 {
    elves.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn solve2(elves: &[Elf]) -> u32 {
    let mut max_calorie_counts = [0, 0, 0];

    for calorie_count in elves.iter().map(|elf| elf.iter().sum()) {
        if calorie_count > max_calorie_counts[0] {
            max_calorie_counts[0] = calorie_count;
            max_calorie_counts.sort();
        }
    };
    max_calorie_counts.iter().sum()
}
