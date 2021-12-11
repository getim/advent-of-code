use std::collections::{HashSet,HashMap};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let file = File::open("input").unwrap();
    let mut lines = BufReader::new(file).lines();
    let numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap()).collect();
    lines.next();
    let mut cards: Vec<BingoCard> = read_cards(&mut lines);

    println!("Result 1: {}", play1(&mut cards, &numbers));
    println!("Result 2: {}", play2(&mut cards, &numbers));
}

fn read_cards(lines: &mut impl Iterator<Item=io::Result<String>>) -> Vec<BingoCard> {
    let mut cards = vec![];
    let mut card_rows = vec![];
    for l in lines {
        let line = l.unwrap();
        if line.is_empty() {
            cards.push(BingoCard::new(card_rows));
            card_rows = vec![];
        } else {
            card_rows.push(
                line.split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                   .collect()
            );
        }
    }
    cards.push(BingoCard::new(card_rows));
    cards
}

fn play1(cards: &mut Vec<BingoCard>, numbers: &Vec<usize>) -> usize {
    for number in numbers {
        for card in cards.iter_mut() {
            card.mark(*number);
            if let Some(score) = card.score(*number) {
                return score
            }
        }
    }
    panic!("Game finished without winner");
}

fn play2(cards: &mut Vec<BingoCard>, numbers: &Vec<usize>) -> usize {
    let cards_count = cards.len();
    let mut cards_finished = HashSet::new();
    for number in numbers {
        for (i, card) in cards.iter_mut().enumerate() {
            card.mark(*number);
            if let Some(score) = card.score(*number) {
                cards_finished.insert(i);
                if cards_finished.len() == cards_count {
                    return score
                }
            }
        }
    }
    panic!("Game finished without losing card completing");
}

#[derive(Debug)]
struct BingoCard {
    values: Vec<Vec<usize>>,
    marked_positions: HashSet<(usize, usize)>,
}

impl BingoCard {
    pub fn new(values: Vec<Vec<usize>>) -> Self {
        Self { values, marked_positions: HashSet::new() }
    }

    pub fn mark(&mut self, number: usize) {
        for i in 0..self.values.len() {
            for j in 0..self.values[i].len() {
                if self.values[i][j] == number {
                    self.marked_positions.insert((i, j));
                }
            }
        }
    }

    pub fn score(&self, last_number: usize) -> Option<usize> {
        let mut x_counts = HashMap::new();
        let mut y_counts = HashMap::new();

        let mut done = false;
        for p in &self.marked_positions {
            *x_counts.entry(p.0).or_insert(0) += 1;
            *y_counts.entry(p.1).or_insert(0) += 1;

            if *x_counts.get(&p.0).unwrap() == self.values.len() ||
                    *y_counts.get(&p.1).unwrap() == self.values.len() {
                done = true;
            }
        }

        if done {
            let mut score = 0;
            for i in 0..self.values.len() {
                for j in 0..self.values.len() {
                    if !self.marked_positions.contains(&(i, j)) {
                        score += self.values[i][j];
                    }
                }
            }
            Some(score * last_number)
        } else {
            None
        }
    }
}


impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-----------------").unwrap();
        for (i, row) in self.values.iter().enumerate() {
            write!(f, "{}", row
                   .iter()
                   .enumerate()
                   .map(|(j, value)| if self.marked_positions.contains(&(i, j)) {
                        highlight(format!("{: >2}", value).as_str())
                   } else {
                        format!("{: >2}", value)
                   })
                   .collect::<Vec<String>>()
                   .join(" ")).unwrap();
            writeln!(f).unwrap();
        }
        write!(f, "-----------------")
    }
}

fn highlight(text: &str) -> String {
    format!("\x1b[0;32;1;4m{}\x1b[0m", text)
}
