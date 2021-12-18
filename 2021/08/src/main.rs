use std::collections::{BTreeSet,HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type Signal = BTreeSet<char>;
type Input = HashSet<Signal>;
type Output = Vec<Signal>;

fn main() {
    let file = File::open("input").unwrap();
    let displays = BufReader::new(file).lines().map(|line| {
        let line = line.unwrap();
        let mut output: Output = line.split_whitespace()
                                          .map(|v| v.chars().collect())
                                          .collect();
        let input: Input = output.drain(0..10).collect();
        output.remove(0);
        (input, output)
    });

    let (mut inputs, outputs): (Vec<Input>, Vec<Output>) = displays.unzip();

    println!("Result 1: {}", solve1(&outputs));
    println!("Result 2: {}", solve2(&mut inputs, &outputs));
}

fn solve1(outputs: &[Output]) -> usize {
    let detect_lengths = [2, 3, 4, 7];

    let mut count = 0;
    for output in outputs {
        for value in output {
            if detect_lengths.contains(&value.len()) {
                count += 1;
            }
        }
    }
    count
}

fn solve2(inputs: &mut Vec<Input>, outputs: &[Output]) -> usize {
    let mut total = 0;
    for (i, input) in inputs.iter_mut().enumerate() {
        let mapping = get_mapping(input);
        total += calculate(&outputs[i], mapping);
    }
    total
}

fn get_mapping(input: &mut Input) -> HashMap<usize, Signal> {
    let mut signal_mapping: HashMap<usize, Signal> = HashMap::with_capacity(10);

    // Unique segment counts
    signal_mapping.insert(1, pop_value_of_length(input, 2));
    signal_mapping.insert(7, pop_value_of_length(input, 3));
    signal_mapping.insert(4, pop_value_of_length(input, 4));
    signal_mapping.insert(8, pop_value_of_length(input, 7));
    // 6 segments
    signal_mapping.insert(9, pop_value_of_length_and_with_common(
            input, 6, signal_mapping.get(&4).unwrap(), 4
    ));
    signal_mapping.insert(0, pop_value_of_length_and_with_common(
            input, 6, signal_mapping.get(&1).unwrap(), 2
    ));
    signal_mapping.insert(6, pop_value_of_length(input, 6));
    // 5 segments
    signal_mapping.insert(3, pop_value_of_length_and_with_common(
            input, 5, signal_mapping.get(&1).unwrap(), 2
    ));
    signal_mapping.insert(2, pop_value_of_length_and_with_common(
            input, 5, signal_mapping.get(&4).unwrap(), 2
    ));
    signal_mapping.insert(5, pop_value_of_length(input, 5));

    signal_mapping
}

fn calculate(output: &Output, mapping: HashMap<usize, Signal>) -> usize {
    let mut output_value = 0;
    let output_length = output.len();
    let base: usize = 10;

    let mut reverse_mapping: HashMap<Signal, usize> = HashMap::with_capacity(mapping.len());
    for (key, value) in mapping {
        reverse_mapping.insert(value, key);
    }

    for (i, value) in output.iter().enumerate() {
        let power_10 = base.pow((output_length - i - 1) as u32);
        output_value += reverse_mapping.get(value).unwrap() * power_10;
    }
    output_value
}

fn pop_value_of_length(input: &mut Input, length: usize) -> Signal {
    let mut matched = Input::new();
    let mut not_matched = Input::new();
    for value in input.drain() {
        if value.len() == length {
            matched.insert(value);
        } else {
            not_matched.insert(value);
        }
    }
    if matched.len() != 1 {
        panic!("Found {} matching signals of length {}, expected 1", matched.len(), length);
    }
    *input = not_matched;
    matched.iter().next().unwrap().clone()
}

fn pop_value_of_length_and_with_common(
    input: &mut Input, length: usize, reference: &Signal, common_count: usize
) -> Signal {
    let mut matched = Input::new();
    let mut not_matched = Input::new();
    for value in input.drain() {
        if value.len() == length && value.intersection(reference).count() == common_count {
            matched.insert(value);
        } else {
            not_matched.insert(value);
        }
    }
    if matched.len() != 1 {
        panic!(
            "Found {} matching signals of length {} and with {} commons with {:?}, \
             expected 1:\n{:?}",
            matched.len(), length, common_count, reference, matched
        );
    }
    *input = not_matched;
    matched.iter().next().unwrap().clone()
}
