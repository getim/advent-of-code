use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::{Add,Sub};
use std::str::FromStr;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Sub<Point> for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct PointParseError {
    _msg: String,
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, PointParseError> {
        let mut split_s = s.split(',');
        let x = read_number(split_s.next())?;
        let y = read_number(split_s.next())?;
        Ok(Point { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn read_number(s: Option<&str>) -> Result<i32, PointParseError> {
    s.ok_or(PointParseError { _msg: "Not enough elements".to_string() })?
        .parse::<i32>().map_err(|e| PointParseError { _msg: e.to_string() })
}

#[derive(Debug)]
struct Scanner {
    id: u32,
    beacons: Vec<Point>,
}

impl Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "--- scanner {} ---", self.id)?;
        for beacon in self.beacons.iter() {
            writeln!(f, "{}", beacon)?;
        }
        write!(f, "")
    }
}

impl Scanner {
    fn construct(lines_vec: &[String]) -> Scanner {
        let mut lines = lines_vec.iter();
        let mut scanner = Scanner {
            id: lines.next().unwrap().split_whitespace().nth(2).unwrap().parse::<u32>().unwrap(),
            beacons: Vec::new(),
        };
        for line in lines {
            scanner.beacons.push(Point::from_str(line).unwrap());
        }
        scanner
    }
}

type ScannerMap = HashMap<Point, Scanner>;

fn read_scanners<B>(lines: std::io::Lines<B>) -> Vec<Scanner> where B: std::io::BufRead {
    let mut scanner_lines = Vec::new();
    let mut scanners = Vec::new();
    for line_result in lines {
        let line = line_result.unwrap();
        if line.is_empty() {
            scanners.push(Scanner::construct(&scanner_lines));
            scanner_lines = Vec::new();
        } else {
            scanner_lines.push(line);
        }
    }
    scanners.push(Scanner::construct(&scanner_lines));
    scanners
}

fn is_valid_overlap(scanner: &Scanner, mapped_scanner: &Scanner, scanner_pos: &Point) -> bool {
    true
}

fn overlay_scanners(mut scanners: Vec<Scanner>) -> ScannerMap {
    let mut scanner_map = HashMap::new();

    scanner_map.insert(Point {x: 0, y: 0}, scanners.pop().unwrap());

    for scanner in scanners {
        'outer: for (mapped_pos, mapped_scanner) in scanner_map {
            for beacon in scanner.beacons {
                for mapped_beacon in &mapped_scanner.beacons {
                    let scanner_pos = mapped_beacon - beacon;
                    if is_valid_overlap(&scanner, &mapped_scanner, &scanner_pos) {
                        scanner_map.insert(mapped_pos + scanner_pos, scanner);
                        break 'outer;
                    }
                }
            }
        }
    }

    scanner_map
}


fn main() {
    let file = File::open("input").unwrap();
    let scanners = read_scanners(BufReader::new(file).lines());
    let scanner_map = overlay_scanners(scanners);

    for (point, scanner) in scanner_map {
        println!("--- {} {}", point, scanner);
    }
}
