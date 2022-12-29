use std::collections::{BinaryHeap,HashMap,HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp;

type Height = char;
type Map = HashMap<Position, Height>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

struct DistancedPosition {
    pos: Position,
    distance: usize,
}

impl Eq for DistancedPosition {
}

impl Ord for DistancedPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for DistancedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl PartialEq for DistancedPosition {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let mut start_pos: Option<Position> = None;
    let mut end_pos: Option<Position> = None;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut map = HashMap::new();
    for (y, line) in BufReader::new(file).lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let pos = Position { x, y };
            if c == 'S' {
                map.insert(pos, 'a');
                start_pos = Some(pos);
            } else if c == 'E' {
                map.insert(pos, 'z');
                end_pos = Some(pos);
            } else {
                map.insert(pos, c);
            }
            max_x = x;
        }
        max_y = y;
    }

    let start_pos = start_pos.unwrap();
    let end_pos = end_pos.unwrap();
    let max_pos = Position { x: max_x, y: max_y };

    println!("Result 1: {}", solve1(&map, start_pos, end_pos, max_pos));
    println!("Result 2: {}", solve2(&map, end_pos, max_pos));
}

fn solve1(map: &Map, start_pos: Position, end_pos: Position, max_pos: Position) -> usize {
    let mut distances = BinaryHeap::new();
    let mut visited = HashSet::new();

    let mut current_pos = start_pos;
    let mut current_distance = 0;

    while current_pos != end_pos {
        for neighbour in get_neighbours(current_pos, max_pos) {
            if !visited.contains(&neighbour) &&
                    map[&neighbour] as u32 <= map[&current_pos] as u32 + 1 {
                distances.push(DistancedPosition {
                    pos: neighbour,
                    distance: current_distance + 1,
                })
            }
        }

        visited.insert(current_pos);
        while visited.contains(&current_pos) {
            let new_pos = distances.pop().unwrap();
            current_distance = new_pos.distance;
            current_pos = new_pos.pos;
        }
    }
    current_distance
}

fn solve2(map: &Map, end_pos: Position, max_pos: Position) -> usize {
    let mut distances = BinaryHeap::new();
    let mut visited = HashSet::new();

    let mut current_pos = end_pos;
    let mut current_distance = 0;

    while map[&current_pos] != 'a' {
        for neighbour in get_neighbours(current_pos, max_pos) {
            if !visited.contains(&neighbour) &&
                    map[&neighbour] as u32 >= map[&current_pos] as u32 - 1 {
                distances.push(DistancedPosition {
                    pos: neighbour,
                    distance: current_distance + 1,
                })
            }
        }

        visited.insert(current_pos);
        while visited.contains(&current_pos) {
            let new_pos = distances.pop().unwrap();
            current_distance = new_pos.distance;
            current_pos = new_pos.pos;
        }
    }
    current_distance
}

fn get_neighbours(pos: Position, max_pos: Position) -> Vec<Position> {
    let mut neighbours = vec![];
    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let x = usize::try_from(pos.x as i32 + dx);
        let y = usize::try_from(pos.y as i32 + dy);
        if let (Ok(x), Ok(y)) = (x, y) {
            if x <= max_pos.x && y <= max_pos.y {
                neighbours.push(Position { x, y });
            }
        }
    }
    neighbours
}
