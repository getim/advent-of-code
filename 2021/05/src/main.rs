use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


type Coord = (i32, i32);
type Pipeline = (Coord, Coord);


fn main() {
    let file = File::open("input").unwrap();
    let lines: Vec<Pipeline> = BufReader::new(file)
        .lines()
        .map(|line| {
            let points: Vec<Coord> = line.unwrap()
                .split(" -> ")
                .map(|point| {
                    let split_point: Vec<&str> = point.split(',').collect();
                    (split_point[0].parse::<i32>().unwrap(),
                     split_point[1].parse::<i32>().unwrap())
                })
                .collect();
            (points[0], points[1])
        })
        .collect();

    println!("{}", solve(&lines, true));
    println!("{}", solve(&lines, false));
}


fn solve(lines: &[Pipeline], skip_diagonals: bool) -> usize {
    let mut counter = HashMap::new();
    for pipeline in lines {
        let x1 = pipeline.0.0;
        let y1 = pipeline.0.1;
        let x2 = pipeline.1.0;
        let y2 = pipeline.1.1;
        if skip_diagonals && x1 != x2 && y1 != y2 {
            continue;
        }

        let x_diff = get_diff(x1, x2);
        let y_diff = get_diff(y1, y2);

        let mut x = x1;
        let mut y = y1;
        while x != x2 + x_diff || y != y2 + y_diff {
            *counter.entry((x, y)).or_insert(0) += 1;
            x += x_diff;
            y += y_diff;
        }
    }
    counter.values().filter(|c| **c > 1).count()
}

fn get_diff(value1: i32, value2: i32) -> i32 {
    match value1.cmp(&value2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }

}
