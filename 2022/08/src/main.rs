use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::num::TryFromIntError;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<(i32, i32)> for Point {
    type Output = Result<Self, TryFromIntError>;

    fn add(self, diff: (i32, i32)) -> Result<Self, TryFromIntError> {
        let result = Point {
            x: usize::try_from(self.x as i32 + diff.0)?,
            y: usize::try_from(self.y as i32 + diff.1)?,
        };
        Ok(result)
    }
}

type Forest = HashMap<Point, usize>;


fn main() {
    let file = File::open("input").unwrap();

    let mut forest = Forest::new();
    let mut x_size = 0;
    let mut y_size = 0;

    for (y, line) in BufReader::new(file).lines().enumerate() {
        y_size = y;
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            forest.insert(Point{x, y}, usize::try_from(c.to_digit(10).unwrap()).unwrap());
            x_size = x;
        }
    }

    let max_point = Point { x: x_size, y: y_size };

    println!("Result 1: {}", solve1(&forest, &max_point));
    println!("Result 2: {}", solve2(&forest, &max_point));
}

fn solve1(forest: &Forest, max_point: &Point) -> usize {
    let mut visible_points = 0;
    for (point, height) in forest {
        if is_visible_from_edge(point, *height, forest, max_point) {
            visible_points += 1;
        }
    }
    visible_points
}

fn solve2(forest: &Forest, max_point: &Point) -> usize {
    let mut max_score = 0;
    for (point, height) in forest {
        max_score = max(max_score, get_scenic_score(point, *height, forest, max_point))
    }
    max_score
}

fn is_visible_from_edge(point: &Point, height: usize, forest: &Forest, max_point: &Point) -> bool {
    for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut neighbour = *point;
        loop {
            let maybe_neighbour = neighbour + direction;
            match maybe_neighbour {
                Ok(n) => {
                    if n.x > max_point.x || n.y > max_point.y {
                        return true;
                    }
                    if forest[&n] >= height {
                        break;
                    }
                    neighbour = n;
                },
                Err(_) => {
                    return true;
                }
            }
        }
    }
    false
}

fn get_scenic_score(point: &Point, height: usize, forest: &Forest, max_point: &Point) -> usize {
    let mut score = 1;
    for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut direction_score = 0;
        let mut neighbour = *point;
        loop {
            let maybe_neighbour = neighbour + direction;
            match maybe_neighbour {
                Ok(n) => {
                    if n.x > max_point.x || n.y > max_point.y   {
                        break;
                    }
                    if forest[&n] >= height {
                        direction_score += 1;
                        break;
                    }
                    neighbour = n;
                },
                Err(_) => {
                    break;
                }
            }
            direction_score += 1;
        }
        score *= direction_score;
    }
    score
}
