use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct CPUState {
    cycle: usize,
    register: i32,
}

impl CPUState {
    fn new() -> Self {
        Self { cycle: 0, register: 1 }
    }
}

struct Display {
    on_pixels: HashSet<usize>,
    row_length: usize,
}

impl Display {
    fn new(row_length: usize) -> Self {
        Display {
            on_pixels: HashSet::new(),
            row_length,
        }
    }

    fn add_pixel(&mut self, state: &CPUState) {
        if (i32::try_from(state.cycle).unwrap() % 40 - state.register).abs() <= 1 {
            self.on_pixels.insert(state.cycle);
        }
    }

    fn draw(&self) {
        for pos in 0..*self.on_pixels.iter().max().unwrap() {
            if pos % self.row_length == 0 {
                println!();
            }
            if self.on_pixels.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let instructions: Vec<Instruction> = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        match split.next().unwrap() {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(split.next().unwrap().parse::<i32>().unwrap()),
            _ => panic!(),
        }
    }).collect();

    println!("Result 1: {}", solve1(&instructions));
    println!("Result 2:");
    solve2(&instructions).draw();
}

fn solve1(instructions: &[Instruction]) -> i32 {
    let mut total_strength = 0;
    let mut state = CPUState::new();

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                state.cycle += 1;
                total_strength += _get_signal_strength_to_add(&state);
            },
            Instruction::Addx(i) => {
                state.cycle += 1;
                total_strength += _get_signal_strength_to_add(&state);
                state.cycle += 1;
                total_strength += _get_signal_strength_to_add(&state);
                state.register += i;
            },
        }
    }

    total_strength
}

fn solve2(instructions: &[Instruction]) -> Display {
    let mut display = Display::new(40);
    let mut state = CPUState::new();

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                display.add_pixel(&state);
                state.cycle += 1;
            },
            Instruction::Addx(i) => {
                display.add_pixel(&state);
                state.cycle += 1;
                display.add_pixel(&state);
                state.cycle += 1;
                state.register += i;
            },
        }
    };
    display
}

fn _get_signal_strength_to_add(state: &CPUState) -> i32 {
    if (state.cycle + 20) % 40 == 0 {
        i32::try_from(state.cycle).unwrap() * state.register
    } else {
        0
    }
}
