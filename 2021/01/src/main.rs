use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::Iterator;


fn main() {
    let file = File::open("input").unwrap();
    let heights: Vec<i32> = BufReader::new(file).lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    println!("Result 1: {}", count_increases(&heights, 1));
    println!("Result 2: {}", count_increases(&heights, 3));
}

fn count_increases(heights: &[i32], window_size: usize) -> usize {
    heights
        .windows(window_size)
        .collect::<Vec<&[i32]>>()
        .windows(2)
        .filter(|windows| windows[1].iter().sum::<i32>() > windows[0].iter().sum::<i32>())
        .count()
}
