use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Line = Vec<char>;
type Stack = Vec<char>;
type Subsystem = Vec<Line>;

fn main() {
    let file = File::open("input").unwrap();
    let mut subsystem: Subsystem = Vec::new();

    for line in BufReader::new(file).lines() {
        subsystem.push(line.unwrap().chars().collect());
    }

    let char_mapping: HashMap<char, char> = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<'),
    ]);

    println!("Result 1: {}", solve1(&subsystem, &char_mapping));
    println!("Result 2: {}", solve2(&subsystem, &char_mapping));
}

fn solve1(
    subsystem: &Subsystem,
    char_mapping: &HashMap<char, char>,
) -> usize {
    let score_mapping: HashMap<char, usize> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let mut total_score = 0;
    for line in subsystem {
        if let (Some(c), _) = get_corrupt_char(line, char_mapping) {
            total_score += score_mapping.get(&c).unwrap();
        }
    }
    total_score
}

fn solve2(subsystem: &Subsystem, char_mapping: &HashMap<char, char>) -> usize {
    let score_mapping: HashMap<char, usize> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    let mut line_scores = Vec::new();
    for line in subsystem {
        match get_corrupt_char(line, char_mapping) {
            (Some(_), _) => (),
            (None, stack) => {
                line_scores.push(get_autocomplete_score(&stack, &score_mapping));
            },
        }
    }
    line_scores.sort();
    line_scores[line_scores.len() / 2]
}

fn get_corrupt_char(line: &Line, char_mapping: &HashMap<char, char>) -> (Option<char>, Stack) {
    let mut stack: Stack = Vec::new();
    for c in line {
        match char_mapping.get(c) {
            Some(opening_c) => {
                if let Some(stack_c) = stack.pop() {
                    if stack_c == *opening_c {
                        continue;
                    }
                }
                return (Some(*c), stack);
            }
            None => stack.push(*c)
        }
    }
    (None, stack)
}

fn get_autocomplete_score(stack: &Stack, score_mapping: &HashMap<char, usize>) -> usize {
    let mut score = 0;
    for c in stack.iter().rev() {
        score = score * 5 + score_mapping.get(c).unwrap();
    }
    score
}
