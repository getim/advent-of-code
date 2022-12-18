use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn touches(&self, other: &Self) -> bool {
        ! ((self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1)
    }

    fn move_towards(&mut self, other: &Self) {
        if self.touches(other) {
            ()
        } else if self.x == other.x {
            self.y += (other.y - self.y).signum();
        } else if self.y == other.y {
            self.x += (other.x - self.x).signum();
        } else {
            self.x += (other.x - self.x).signum();
            self.y += (other.y - self.y).signum();
        }
    }

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        match input {
            "D" => Ok(Down),
            "L" => Ok(Left),
            "R" => Ok(Right),
            "U" => Ok(Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    count: usize,
}


fn main() {
    let file = File::open("input").unwrap();
    let commands: Vec<Command> = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        Command {
            dir: Direction::from_str(split.next().unwrap()).unwrap(),
            count: split.next().unwrap().parse::<usize>().unwrap(),
        }
    }).collect();

    println!("Result 1: {}", solve(&commands, 2));
    println!("Result 2: {}", solve(&commands, 10));
}

fn solve(commands: &[Command], rope_length: usize) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut knots = vec![Position::new(); rope_length];

    for command in commands {
        for _ in 0..command.count {
            knots[0].move_dir(&command.dir);
            for i in 1..knots.len() {
                let previous_knot = knots[i - 1];
                knots[i].move_towards(&previous_knot);
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    visited.len()
}
