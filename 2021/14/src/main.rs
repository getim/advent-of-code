use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Element = char;
type ElementPair = (Element, Element);
type Polymer = Vec<Element>;
type Ruleset = HashMap<ElementPair, Element>;
type ElementCounter = HashMap<Element, usize>;
type Memory = HashMap<(ElementPair, usize), ElementCounter>;

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = BufReader::new(file).lines();

    let polymer: Polymer = lines.next().unwrap().unwrap().chars().collect();
    lines.next();

    let mut rules = Ruleset::new();
    for line in lines {
        let chars: Vec<_> = line.unwrap().chars().collect();
        rules.insert((chars[0], chars[1]), chars[6]);
    }

    println!("Result 1: {}", solve1(&polymer, &rules));
    println!("Result 2: {}", solve2(&polymer, &rules));
}

fn solve1(polymer: &Polymer, rules: &Ruleset) -> usize {
    expand_and_count(polymer, rules, 10)
}

fn solve2(polymer: &Polymer, rules: &Ruleset) -> usize {
    expand_and_count(polymer, rules, 40)
}

fn expand_and_count(polymer: &Polymer, rules: &Ruleset, generations: usize) -> usize {
    let mut memory = Memory::new();
    let mut first_run = true;
    let mut counters = Vec::new();
    for pair in polymer.windows(2) {
        let mut counter = get_element_count(&(pair[0], pair[1]), rules, generations, &mut memory);
        // We will double-count all elements except for the first and last.
        // Always adjust first of pair, therefore not adjusting the last overall element,
        // and skip it in the first iteration to not do it for the first overall element.
        if !first_run {
            *counter.get_mut(&pair[0]).unwrap() -= 1;
        } else {
            first_run = false
        }
        counters.push(counter);
    }

    let overall_counter = combine_counters(counters);
    overall_counter.values().max().unwrap() - overall_counter.values().min().unwrap()
}

fn get_element_count(
    pair: &ElementPair, rules: &Ruleset, generations: usize, memory: &mut Memory
) -> ElementCounter {
    if generations == 0 {
        get_base_element_count(pair)
    } else if let Some(result) = memory.get(&(*pair, generations)) {
        result.clone()
    } else if let Some(middle_element) = rules.get(pair) {
        let mut result = combine_counters(vec![
            get_element_count(&(pair.0, *middle_element), rules, generations - 1, memory),
            get_element_count(&(*middle_element, pair.1), rules, generations - 1, memory),
        ]);
        // The two calls together will double-count the middle element.
        *result.get_mut(middle_element).unwrap() -= 1;
        memory.insert((*pair, generations), result.clone());
        result
    } else {
        get_base_element_count(pair)
    }
}

fn get_base_element_count(pair: &ElementPair) -> ElementCounter {
    if pair.0 == pair.1 {
        HashMap::from([(pair.0, 2)])
    } else {
        HashMap::from([(pair.0, 1), (pair.1, 1)])
    }
}

fn combine_counters(counters: Vec<ElementCounter>) -> ElementCounter {
    let mut result = HashMap::new();
    for counter in counters {
        for (key, value) in counter {
            *result.entry(key).or_insert(0) += value;
        }
    }
    result
}
