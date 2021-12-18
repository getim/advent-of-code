use std::cmp::Ordering;
use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Position = (usize, usize);
type Height = u32;
type Grid = HashMap<Position, Height>;
type Basin = HashSet<Position>;

fn main() {
    let file = File::open("input").unwrap();
    let mut grid: Grid = HashMap::new();
    let mut grid_width = 0;
    let mut grid_height = 0;
    for (i, line) in BufReader::new(file).lines().enumerate() {
        grid_width = i;
        for (j, height) in line.unwrap().chars().enumerate() {
            grid.insert((i, j), height.to_digit(10).unwrap());
            grid_height = j
        }
    }
    grid_width += 1;
    grid_height += 1;

    let low_points = get_low_points(&grid, grid_width, grid_height);

    println!("Result 1: {}", solve1(&grid, &low_points));
    println!("Result 2: {}", solve2(&grid, &low_points))
}

fn solve1(grid: &Grid, low_points: &[Position]) -> u32 {
    let mut risk_level = 0;
    for low_point in low_points {
        risk_level += 1 + grid.get(low_point).unwrap();
    }
    risk_level
}

fn solve2(grid: &Grid, low_points: &[Position]) -> usize {
    let mut basin_sizes = Vec::with_capacity(low_points.len());
    for low_point in low_points {
        let mut basin = HashSet::new();
        extend_basin(&mut basin, low_point, grid);
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn get_low_points(grid: &Grid, grid_width: usize, grid_height: usize) -> Vec<Position> {
    let mut low_points = Vec::new();
    for i in 0..grid_width {
        for j in 0..grid_height {
            let (lower_pos, equal_pos, _) = get_neighbours(&(i, j), grid);
            if lower_pos.is_empty() && equal_pos.is_empty() {
                low_points.push((i, j));
            }
        }
    }
    low_points
}

/// Returns vectors of lower, equal and higher neighbours.
fn get_neighbours(pos: &Position, grid: &Grid) -> (Vec<Position>, Vec<Position>, Vec<Position>) {
    let mut lower_pos = Vec::new();
    let mut equal_pos = Vec::new();
    let mut higher_pos = Vec::new();
    let height = grid.get(pos).unwrap();
    for (i_diff, j_diff) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        if let Ok(neighbour_i) = (pos.0 as i32 + i_diff).try_into() {
            if let Ok(neighbour_j) = (pos.1 as i32 + j_diff).try_into() {
                if let Some(neighbour_height) = grid.get(&(neighbour_i, neighbour_j)) {
                    match neighbour_height.cmp(height) {
                        Ordering::Less => lower_pos.push((neighbour_i, neighbour_j)),
                        Ordering::Equal => equal_pos.push((neighbour_i, neighbour_j)),
                        Ordering::Greater => higher_pos.push((neighbour_i, neighbour_j)),
                    }
                }
            }
        }
    }
    (lower_pos, equal_pos, higher_pos)
}

fn extend_basin(basin: &mut Basin, pos: &Position, grid: &Grid) {
    if *grid.get(pos).unwrap() < 9 && !basin.contains(pos) {
        basin.insert(*pos);

        let (_, equal_pos, higher_pos) = get_neighbours(pos, grid);
        for pos in equal_pos.iter().chain(higher_pos.iter()) {
            extend_basin(basin, pos, grid);
        }
    }
}
