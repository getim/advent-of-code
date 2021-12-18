use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::min;


fn main() {
    let file = File::open("input").unwrap();
    let mut crabs: Vec<i32> = BufReader::new(file)
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    crabs.sort();
    println!("Result 1: {}", solve1(&crabs));
    println!("Result 2: {}", solve2(&crabs))
}

fn solve1(crabs: &[i32]) -> i32 {
    let index = crabs.len() as f32 / 2.0;
    let index_trunc = index.trunc();
    let median: i32 = if index == index_trunc {
        crabs[index as usize]
    } else {
        (crabs[index_trunc as usize] + crabs[index_trunc as usize + 1]) / 2
    };

    crabs.iter().map(|c| (c - median).abs()).sum()
}

fn solve2(crabs: &[i32]) -> i32 {
    let sol1: i32 = (crabs.iter().sum::<i32>() as f32 / crabs.len() as f32).trunc() as i32;
    let sol2 = sol1 + 1;
    let fuel1 = calculate_fuel_part_2(crabs, sol1);
    let fuel2 = calculate_fuel_part_2(crabs, sol2);
    min(fuel1, fuel2)
}


fn calculate_fuel_part_2(crabs: &[i32], level: i32) -> i32 {
    crabs.iter()
        .map(|c| { let distance = (c - level).abs(); distance * (distance + 1) / 2 })
        .sum::<i32>()
}
