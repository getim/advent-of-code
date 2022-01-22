use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug,Clone,Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug,Clone,Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum MissReason {
    Overshot,
    Undershot,
    YTooHigh,
    XTooHigh,
}

#[derive(Debug)]
struct ShotMiss {
    reason: MissReason,
}

static TOO_HIGHS_ALLOWED: i32 = 10000;


fn main() {
    let file = File::open("input").unwrap();
    let line = BufReader::new(file).lines().next().unwrap().unwrap();
    let mut words = line.split_whitespace();

    words.next();
    words.next();
    let (x_min, x_max) = get_numbers(words.next().unwrap());
    let (y_min, y_max) = get_numbers(words.next().unwrap());
    let p_min = Point { x: x_min, y: y_min };
    let p_max = Point { x: x_max, y: y_max };

    println!("Result 1: {}", solve1(p_min, p_max));
    println!("Result 2: {}", solve2(p_min, p_max));
}

fn solve1(p_min: Point, p_max: Point) -> i32 {
    try_smart_shots(p_min, p_max)
}

fn solve2(p_min: Point, p_max: Point) -> i32 {
    try_all_the_shots(p_min, p_max)
}

/* This is trying way too hard to be smart but then needs to fall back to specifying a
 * TOO_HIGHS_ALLOWED anyway. From a shot that goes straight through the target horizontally
 * or vertically, we can't conclude that all higher horizontal or vertical velocities
 * respectively are going to be too high as well.
 * But it's still fun :)
 */
fn try_smart_shots(p_min: Point, p_max: Point) -> i32 {
    let mut max_y = i32::MIN;
    let mut v_y = p_min.y - 1;
    let mut y_too_highs_allowed = TOO_HIGHS_ALLOWED;
    'outer: loop {
        v_y += 1;
        let mut v_x = 1;
        let mut x_too_highs_allowed = TOO_HIGHS_ALLOWED;
        loop {
            if x_too_highs_allowed == 0 {
                y_too_highs_allowed = TOO_HIGHS_ALLOWED;
                break
            }
            if y_too_highs_allowed == 0 {
                break 'outer;
            }
            v_x += 1;
            let v = Velocity { x: v_x, y: v_y };
            match smart_shoot(v, p_min, p_max) {
                Ok(current_max_y) => {
                    if current_max_y > max_y {
                        max_y = current_max_y;
                    }
                }
                Err(e) => {
                    match e.reason {
                        MissReason::Overshot => {
                            y_too_highs_allowed = TOO_HIGHS_ALLOWED;
                            break;
                        }
                        MissReason::Undershot => {
                            y_too_highs_allowed = TOO_HIGHS_ALLOWED;
                            continue;
                        }
                        MissReason::XTooHigh => {
                            x_too_highs_allowed -= 1;
                        }
                        MissReason::YTooHigh => {
                            y_too_highs_allowed -= 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    max_y
}

fn try_all_the_shots(p_min: Point, p_max: Point) -> i32 {
    let mut total_hits = 0;
    for v_x in 1..1000 {
        for v_y in -1000..1000 {
            if dumb_shoot(Velocity { x: v_x, y: v_y }, p_min, p_max) {
                total_hits += 1;
            }
        }
    }
    total_hits
}

fn get_numbers(mut word: &str) -> (i32, i32) {
    if let Some(stripped) = word.strip_suffix(',') {
        word = stripped;
    }
    word = &word[2..];
    let mut split_word = word.split('.');
    let first = split_word.next().unwrap().parse::<i32>().unwrap();
    split_word.next();
    let second = split_word.next().unwrap().parse::<i32>().unwrap();
    (first, second)
}

fn smart_shoot(mut velocity: Velocity, p_min: Point, p_max: Point) -> Result<i32, ShotMiss> {
    let mut probe = Point { x: 0, y: 0 };
    let mut max_y = 0;
    let mut above = true;
    let mut in_front = true;
    loop {
        probe.x += velocity.x;
        probe.y += velocity.y;
        if velocity.x > 0 {
            velocity.x -= 1;
        }
        velocity.y -= 1;

        if probe.y < p_min.y {
            if probe.x > p_max.x {
                if in_front {
                    return Err(ShotMiss { reason: MissReason::XTooHigh });
                } else {
                    return Err(ShotMiss { reason: MissReason::Overshot });
                }
            }
            if above {
                return Err(ShotMiss { reason: MissReason::YTooHigh });
            } else {
                return Err(ShotMiss { reason: MissReason::Undershot });
            }
        }

        if probe.x >= p_min.x {
            in_front = false;
        }
        if probe.y <= p_max.y {
            above = false;
        }

        if probe.y > max_y {
            max_y = probe.y
        }

        if p_min.x <= probe.x && probe.x <= p_max.x &&
                p_min.y <= probe.y && probe.y <= p_max.y {
            return Ok(max_y)
        }
    }
}

fn dumb_shoot(mut velocity: Velocity, p_min: Point, p_max: Point) -> bool {
    let mut probe = Point { x: 0, y: 0 };
    loop {
        probe.x += velocity.x;
        probe.y += velocity.y;
        if velocity.x > 0 {
            velocity.x -= 1;
        }
        velocity.y -= 1;

        if probe.y < p_min.y {
            return false;
        }

        if p_min.x <= probe.x && probe.x <= p_max.x &&
                p_min.y <= probe.y && probe.y <= p_max.y {
            return true;
        }
    }
}
