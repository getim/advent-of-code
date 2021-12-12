use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


const CYCLE_AGE: usize = 6;
const EXTRA_YOUNG_AGE: usize = 2;


fn main() {
    let file = File::open("input").unwrap();
    let lanterns: Vec<usize> = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut lanterns_by_age = [0].repeat(CYCLE_AGE + EXTRA_YOUNG_AGE + 1);
    for fish in lanterns {
        lanterns_by_age[fish] += 1;
    }

    println!("Result 1: {}", solve(&mut lanterns_by_age.clone(), 80));
    println!("Result 2: {}", solve(&mut lanterns_by_age, 256));
}


fn solve(lanterns: &mut Vec<usize>, day_count: usize) -> usize {
    for _ in 0..day_count {
        let creating_lanterns_count = lanterns[0];
        lanterns.rotate_left(1);
        lanterns[CYCLE_AGE] += creating_lanterns_count;
    }
    lanterns.iter().sum::<usize>()
}
