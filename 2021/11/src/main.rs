use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Octopus = usize;
type Position = (usize, usize);
type Grid = HashMap<Position, Octopus>;

fn main() {
    let file = File::open("input").unwrap();
    let mut grid: Grid = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (i, line) in BufReader::new(file).lines().enumerate() {
        width += 1;
        for (j, c) in line.unwrap().chars().enumerate() {
            grid.insert((i, j), c.to_digit(10).unwrap() as usize);
            height += 1;
        }
    }
    height /= width;

    println!("Result 1: {}", solve1(&mut grid.clone(), width, height));
    println!("Result 2: {}", solve2(&mut grid, width, height));
}

fn solve1(grid: &mut Grid, width: usize, height: usize) -> usize {
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += step(grid, width, height);
    }
    flash_count
}

fn solve2(grid: &mut Grid, width: usize, height: usize) -> usize {
    let grid_size = width * height;
    let mut steps = 0;
    loop {
        steps += 1;
        if step(grid, width, height) == grid_size {
            break;
        }
    }
    steps
}

fn step(grid: &mut Grid, width: usize, height: usize) -> usize {
    let mut flashes = HashSet::new();
    for i in 0..width {
        for j in 0..height {
            increment_and_check(grid, &(i, j), &mut flashes);
        }
    }
    for flash in &flashes {
        grid.insert(*flash, 0);
    }
    flashes.len()
}

fn increment_and_check(grid: &mut Grid, pos: &Position, flashes: &mut HashSet<Position>) {
    if !flashes.contains(pos) {
        if let Some(value) = grid.get(pos) {
            let new_value = value + 1;
            grid.insert(*pos, new_value);

            if new_value > 9 {
                flashes.insert(*pos);
                for i_diff in [-1, 0, 1] {
                    for j_diff in [-1, 0, 1] {
                        if i_diff == 0 && j_diff == 0 {
                            continue
                        }
                        if let Ok(neighbour_i) = (pos.0 as i32 + i_diff).try_into() {
                            if let Ok(neighbour_j) = (pos.1 as i32 + j_diff).try_into() {
                                increment_and_check(grid, &(neighbour_i, neighbour_j), flashes);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn print_grid(grid: &Grid, flashes: HashSet<Position>, width: usize, height: usize) {
    println!("{:-<1$}", "", width + 2);
    for i in 0..width {
        print!("|");
        for j in 0..height {
            let value = grid.get(&(i, j)).unwrap();
            if flashes.contains(&(i, j)) {
                print!("\u{001b}[46;1m{}\u{001b}[0m", value);
            } else {
                print!("{}", value);
            }
        }
        println!("|");
    }
    println!("{:-<1$}", "", width + 2);
}
