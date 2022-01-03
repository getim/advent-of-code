use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


type Position = (usize, usize);
type Risk = u32;
type Cave = HashMap<Position, Risk>;

#[derive(Eq,Debug)]
struct ScoredPosition {
    position: Position,
    total_risk: u32,
}

fn main() {
    let file = File::open("input").unwrap();

    let mut len_i = 0;
    let mut len_j = 0;
    let mut cave = Cave::new();
    for (i, line) in BufReader::new(file).lines().enumerate() {
        len_i += 1;
        for (j, c) in line.unwrap().chars().enumerate() {
            len_j += 1;
            cave.insert((i, j), c.to_digit(10).unwrap());
        }
    }
    len_j /= len_i;

    println!("Result 1: {}", solve1(&cave, len_i, len_j));
    println!("Result 2: {}", solve2(&cave, len_i, len_j));
}

fn solve1(cave: &Cave, len_i: usize, len_j: usize) -> u32 {
    search_path(cave, len_i, len_j, |c, p| c.get(p).copied())
}

fn solve2(cave: &Cave, len_i: usize, len_j: usize) -> u32 {
    search_path(cave, len_i * 5, len_j * 5, |c, p| risk_getter2(c, p, len_i, len_j, 5))
}

/// It's probably faster to calculate the whole grid in advance but this is fun :)
fn risk_getter2(
    cave: &Cave,
    position: &Position,
    base_len_i: usize,
    base_len_j: usize,
    repeats: usize,
) -> Option<Risk> {
    let repeat_i = position.0 / base_len_i;
    let base_i = position.0 - repeat_i * base_len_i;
    let repeat_j = position.1 / base_len_j;
    let base_j = position.1 - repeat_j * base_len_j;

    if repeat_i > repeats - 1 || repeat_j > repeats - 1 {
        None
    } else {
        let result = cave.get(&(base_i, base_j))
                     .map(|r| (r + repeat_i as u32 + repeat_j as u32 - 1).rem_euclid(9) + 1);
        result
    }
}

fn search_path<F>(
    cave: &Cave,
    len_i: usize,
    len_j: usize,
    risk_getter: F,
) -> u32
where F: Fn(&Cave, &Position) -> Option<Risk> {
    let mut current_pos = (0, 0);
    let mut current_risk = 0;
    let mut heaped_risks: BinaryHeap<ScoredPosition> =
        (0..len_i)
        .map(|i| (0..len_j)
            .map(move |j| ScoredPosition {
                position: (i, j),
                total_risk: if (i, j) == current_pos { 0 } else { u32::MAX },
            })
        )
        .flatten()
        .collect();
    let mut hashed_risks = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();

    loop {
        for (i_diff, j_diff) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if let Ok(neighbour_i) = (current_pos.0 as i32 + i_diff).try_into() {
                if let Ok(neighbour_j) = (current_pos.1 as i32 + j_diff).try_into() {
                    let neighbour = &(neighbour_i, neighbour_j);
                    if visited.contains(neighbour) {
                        continue
                    }
                    if let Some(neighbour_risk) = risk_getter(cave, neighbour) {
                        let neighbour_total_risk = hashed_risks.get(neighbour).unwrap_or(&u32::MAX);
                        let new_risk = current_risk + neighbour_risk;
                        if new_risk < *neighbour_total_risk {
                            heaped_risks.push(ScoredPosition {
                                position: *neighbour,
                                total_risk: new_risk
                            });
                            hashed_risks.insert(*neighbour, new_risk);
                        }
                    }
                }
            }
        }

        visited.insert(current_pos);
        let new_current = heaped_risks.pop().unwrap();
        current_pos = new_current.position;
        current_risk = new_current.total_risk;

        if current_pos == (len_i - 1, len_j - 1) {
            return current_risk
        }
    }
}

impl Ord for ScoredPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_risk.cmp(&other.total_risk).reverse()
    }
}

impl PartialOrd for ScoredPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ScoredPosition {
    fn eq(&self, other: &Self) -> bool {
        self.total_risk == other.total_risk
    }
}
