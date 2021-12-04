use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let file = File::open("input").unwrap();
    let commands: Vec<(String, i32)> = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let dir = words.next().unwrap();
        let count = words.next().unwrap().parse::<i32>().unwrap();
        (dir.to_string(), count)
    }).collect();

    println!("Result 1: {}", solve1(&commands));
    println!("Result 2: {}", solve2(&commands));
}

fn solve1(commands: &[(String, i32)]) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for (dir, count) in commands {
        match dir.as_str() {
            "forward" => x += count,
            "down" => y += count,
            "up" => y -= count,
            _ => panic!("Bad direction: {}", dir),
        }
    }

    x * y
}

fn solve2(commands: &[(String, i32)]) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;

    for (dir, count) in commands {
        match dir.as_str() {
            "forward" => {
                x += count;
                y += aim * count;
            },
            "down" => aim += count,
            "up" => aim -= count,
            _ => panic!("Bad direction: {}", dir),
        }
    }

    x * y
}
