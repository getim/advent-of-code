use std::cmp::{max,min};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<&(i32, i32)> for Point {
    type Output = Self;

    fn add(self, diff: &(i32, i32)) -> Self {
        Point {
            x: usize::try_from(self.x as i32 + diff.0).unwrap(),
            y: usize::try_from(self.y as i32 + diff.1).unwrap(),
        }
    }
}

struct Cave {
    rocks: HashSet<Point>,
    sands: HashSet<Point>,
    bottom_y: usize,
}

impl Cave {
    fn is_occupied(&self, point: &Point) -> bool {
        self.rocks.contains(point) || self.sands.contains(point)
    }

    fn drop_sand1(&mut self) -> Option<Point> {
        let mut sand = Point { x: 500, y: 0 };
        loop {
            let mut sand_moved = false;
            for sand_diff in [(0, 1), (-1, 1), (1, 1)] {
                let new_sand = sand + &sand_diff;
                if !self.is_occupied(&new_sand) {
                    sand = new_sand;
                    sand_moved = true;
                    break;
                }
            }
            if sand.y > self.bottom_y {
                return None
            }
            if !sand_moved {
                break
            }
        }
        self.sands.insert(sand);
        Some(sand)
    }

    fn drop_sand2(&mut self) -> Option<Point> {
        let mut sand = Point { x: 500, y: 0 };
        loop {
            let mut sand_moved = false;
            for sand_diff in [(0, 1), (-1, 1), (1, 1)] {
                let new_sand = sand + &sand_diff;
                if !self.is_occupied(&new_sand) && new_sand.y < self.bottom_y + 2 {
                    sand = new_sand;
                    sand_moved = true;
                    break;
                }
            }
            if sand.y == 0 && self.is_occupied(&sand) {
                return None
            }
            if !sand_moved {
                break;
            }
        }
        self.sands.insert(sand);
        Some(sand)
    }
}


fn main() {
    let file = File::open("input").unwrap();
    let mut rocks = HashSet::new();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        rocks.extend(parse_line(line));
    }
    let bottom_y = rocks.iter().map(|p| p.y).max().unwrap();
    let mut cave = Cave { rocks, sands: HashSet::new(), bottom_y };

    println!("Result 1: {}", solve1(&mut cave));
    cave.sands.clear();
    println!("Result 2: {}", solve2(&mut cave));
}

fn parse_line(line: String) -> HashSet<Point> {
    let mut result = HashSet::new();
    let mut previous_point: Option<Point> = None;
    for raw_point in line.split(" -> ") {
        let mut split_point = raw_point.split(',');
        let point = Point {
            x: split_point.next().unwrap().parse::<usize>().unwrap(),
            y: split_point.next().unwrap().parse::<usize>().unwrap(),
        };

        if let Some(prev) = previous_point {
            for x in min(point.x, prev.x)..=max(point.x, prev.x) {
                for y in min(point.y, prev.y)..=max(point.y, prev.y) {
                    result.insert(Point { x, y });
                }
            }
        }
        previous_point = Some(point);
    }
    result
}

fn solve1(cave: &mut Cave) -> usize {
    loop {
        if cave.drop_sand1().is_none() {
            break;
        }
    }
    cave.sands.len()
}

fn solve2(cave: &mut Cave) -> usize {
    loop {
        if cave.drop_sand2().is_none() {
            break;
        }
    }
    cave.sands.len()
}
