use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn get_winning_move(&self) -> Self {
        use Move::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn get_losing_move(&self) -> Self {
        use Move::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn from_char(c: char) -> Self {
        use Move::*;
        match c {
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissors,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn from_char(c: char) -> Self {
        use Outcome::*;
        match c {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Game {
    you: Move,
    elf: Move,
}

impl Game {
    fn get_outcome(&self) -> Outcome {
        if self.you == self.elf {
            Outcome::Draw
        } else if self.elf.get_winning_move() == self.you {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn get_points(&self) -> u32 {
        let move_points = match self.you {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };
        let outcome_points = match self.get_outcome() {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };
        move_points + outcome_points
    }
}

fn main() {
    println!("Result 1: {}", solve1());
    println!("Result 2: {}", solve2());
}

fn solve1() -> u32 {
    let file = File::open("input").unwrap();
    let games: Vec<Game> = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut chars = line.chars();
        let elf_move = Move::from_char(chars.next().unwrap());
        Game {
            elf: elf_move,
            you: Move::from_char(chars.nth(1).unwrap()),
        }
    }).collect();

    games.iter().map(|game| game.get_points()).sum()
}

fn solve2() -> u32 {
    let file = File::open("input").unwrap();
    let games: Vec<Game> = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut chars = line.chars();
        let elf_move = Move::from_char(chars.next().unwrap());
        Game {
            elf: elf_move,
            you: match Outcome::from_char(chars.nth(1).unwrap()) {
                Outcome::Win => elf_move.get_winning_move(),
                Outcome::Draw => elf_move,
                Outcome::Loss => elf_move.get_losing_move(),
            }
        }
    }).collect();

    games.iter().map(|game| game.get_points()).sum()
}
