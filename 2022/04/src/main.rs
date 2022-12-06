use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


#[derive(Debug)]
struct Elf {
    lower: u32,
    upper: u32,
}

#[derive(Debug)]
struct Pair {
    first: Elf,
    second: Elf,
}


impl Elf {
    fn contains(&self, other: &Elf) -> bool {
        self.lower <= other.lower && other.upper <= self.upper
    }

    fn contains_bound_of(&self, other: &Elf) -> bool {
        self.lower <= other.lower && other.lower <= self.upper ||
            self.lower <= other.upper && other.upper <= self.upper
    }

    fn from_input(text_input: &str) -> Self {
        let mut split = text_input.split('-');
        Elf {
            lower: split.next().unwrap().parse::<u32>().unwrap(),
            upper: split.next().unwrap().parse::<u32>().unwrap(),
        }
    }
}

impl Pair {
    fn has_contain(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn has_overlap(&self) -> bool {
        self.first.contains_bound_of(&self.second) ||
            self.second.contains_bound_of(&self.first)
    }

    fn from_input(text_input: &str) -> Self {
        let mut split = text_input.split(',');
        Pair {
            first: Elf::from_input(split.next().unwrap()),
            second: Elf::from_input(split.next().unwrap()),
        }
    }
}


fn main() {
    let file = File::open("input").unwrap();
    let pairs: Vec<Pair> = BufReader::new(file).lines().map(|line| {
        Pair::from_input(&line.unwrap())
    }).collect();

    println!("Result 1: {}", solve1(&pairs));
    println!("Result 2: {}", solve2(&pairs));
}


fn solve1(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|p| p.has_contain()).count()
}


fn solve2(pairs: &[Pair]) -> usize {
    pairs.iter().filter(|p| p.has_overlap()).count()
}
