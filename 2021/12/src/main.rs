use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Cave = String;
type System = HashMap<Cave, HashSet<Cave>>;
type Route = Vec<Cave>;

fn main() {
    let file = File::open("input").unwrap();
    let mut system = HashMap::new();

    for line in BufReader::new(file).lines() {
        let split_line: Vec<String> = line.unwrap().split('-').map(|s| s.to_owned()).collect();
        system.entry(split_line[0].clone()).or_insert_with(HashSet::new)
            .insert(split_line[1].clone());
        system.entry(split_line[1].clone()).or_insert_with(HashSet::new)
            .insert(split_line[0].clone());
    }

    println!("Result 1: {}", solve1(&system));
    println!("Result 2: {}", solve2(&system));
}

fn solve1(system: &System) -> usize {
    let mut route = vec!["start".to_owned()];
    let mut visited = HashSet::from(["start".to_owned()]);
    let mut completed_routes = HashSet::new();
    search_routes(&mut route, system, &mut visited, &mut completed_routes, true);
    completed_routes.len()
}

fn solve2(system: &System) -> usize {
    let mut route = vec!["start".to_owned()];
    let mut visited = HashSet::from(["start".to_owned()]);
    let mut completed_routes = HashSet::new();
    search_routes(&mut route, system, &mut visited, &mut completed_routes, false);
    completed_routes.len()
}

fn search_routes(route: &mut Route, system: &System, visited: &mut HashSet<Cave>,
                 routes_completed: &mut HashSet<Route>, double_visit_done: bool) {
    let current_cave = route.last().unwrap();
    if current_cave == "end" {
        routes_completed.insert(route.clone());
        return
    }
    for next_cave in system.get(current_cave).unwrap() {
        let mut new_visited = visited.clone();
        let mut using_double_visit = false;
        if is_small(next_cave) {
            if visited.contains(next_cave) {
                if double_visit_done || next_cave == "start" {
                    continue;
                } else {
                    using_double_visit = true;
                }
            }
            new_visited.insert(next_cave.clone());
        }
        let mut new_route = route.clone();
        new_route.push(next_cave.clone());
        search_routes(&mut new_route, system, &mut new_visited, routes_completed,
                      double_visit_done || using_double_visit);
    }
}

fn is_small(cave: &Cave) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}
