use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Dot = (usize, usize);
type Paper = HashSet<Dot>;
struct Instruction {
    axes: char,
    location: usize,
}
type Instructions = Vec<Instruction>;

fn main() {
    let file = File::open("input").unwrap();
    let mut paper = Paper::new();
    let mut instructions = Instructions::new();
    let mut reading_dots = true;

    for l in BufReader::new(file).lines() {
        let line = l.unwrap();
        if line.is_empty() {
            reading_dots = false;
            continue;
        }
        if reading_dots {
            let split_line: Vec<_> = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            paper.insert((split_line[0], split_line[1]));
        } else {
            let parts: Vec<_> = line.split_whitespace().last().unwrap().split('=').collect();
            instructions.push(Instruction {
                axes: parts[0].chars().next().unwrap(),
                location: parts[1].parse::<usize>().unwrap(),
            })
        }
    }

    println!("Result 1: {}", solve1(&mut paper.clone(), &instructions));
    solve2(&mut paper, &instructions);
}

fn solve1(paper: &mut Paper, instructions: &Instructions) -> usize {
    fold(paper, instructions.get(0).unwrap());
    paper.len()
}

fn solve2(paper: &mut Paper, instructions: &Instructions) {
    for instruction in instructions {
        fold(paper, instruction);
    }
    print(paper);
}

fn fold(paper: &mut Paper, instruction: &Instruction) {
    let mut new_paper: Paper = Paper::new();
    for dot in paper.iter() {
        if instruction.axes == 'x' {
            if dot.0 < instruction.location {
                new_paper.insert(*dot);
            } else {
                new_paper.insert((2 * instruction.location - dot.0, dot.1));
            }
        } else if dot.1 < instruction.location {
                new_paper.insert(*dot);
        } else {
            new_paper.insert((dot.0, 2 * instruction.location - dot.1));
        }
    }
    *paper = new_paper;
}

fn print(paper: &Paper) {
    let mut max_x = 0;
    let mut max_y = 0;
    for dot in paper {
        max_x = max(max_x, dot.0);
        max_y = max(max_y, dot.1);
    }

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
