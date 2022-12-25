use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

type WorryFunction<'a> = Box<dyn Fn(u64) -> u64 + 'a>;

struct Monkey<'a> {
    items: Vec<u64>,
    worry_function: WorryFunction<'a>,
    test_mod: u64,
    true_dest: usize,
    false_dest: usize,
}

impl fmt::Debug for Monkey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Monkey with {:?}", self.items)
    }
}

impl Monkey<'_> {
    fn from_lines(lines: &mut std::io::Lines<BufReader<File>>) -> Self {
        let items = _get_next_input_values(lines)
            .trim().split(", ").map(|n| n.parse::<u64>().unwrap()).collect();
        let worry_function = _parse_worry_function(
            _get_next_input_values(lines).split(" = ").last().unwrap().to_owned()
        );
        let test_mod = _get_next_input_values(lines)
            .split(" by ").last().unwrap().parse::<u64>().unwrap();
        let true_dest = _get_next_input_values(lines)
            .split(" monkey ").last().unwrap().parse::<usize>().unwrap();
        let false_dest = _get_next_input_values(lines)
            .split(" monkey ").last().unwrap().parse::<usize>().unwrap();
        lines.next();
        Self {
            items,
            worry_function,
            test_mod,
            true_dest,
            false_dest,
        }
    }
}


fn main() {
    println!("Result 1: {}", solve1(&mut _get_monkeys()));
    println!("Result 2: {}", solve2(&mut _get_monkeys()));
}

fn solve1(monkeys: &mut Vec<Monkey>) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let new_item = (*monkeys[i].worry_function)(monkeys[i].items[j]) / 3;
                let dest = if new_item % monkeys[i].test_mod == 0 {
                    monkeys[i].true_dest
                } else {
                    monkeys[i].false_dest
                };
                monkeys[dest].items.push(new_item);
                inspections[i] += 1;
            }
            monkeys[i].items.clear();
        };
    };

    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn solve2(monkeys: &mut Vec<Monkey>) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    let common_mod: u64 = monkeys.iter().map(|m| m.test_mod).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let new_item = (*monkeys[i].worry_function)(monkeys[i].items[j]) % common_mod;
                let dest = if new_item % monkeys[i].test_mod == 0 {
                    monkeys[i].true_dest
                } else {
                    monkeys[i].false_dest
                };
                monkeys[dest].items.push(new_item);
                inspections[i] += 1;
            }
            monkeys[i].items.clear();
        };
    };

    inspections.sort();
    inspections.pop().unwrap() * inspections.pop().unwrap()
}

fn _get_monkeys<'a>() -> Vec<Monkey<'a>> {
    let file = File::open("input").unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut monkeys: Vec<Monkey> = vec![];
    while let Some(_) = lines.next() {
        monkeys.push(Monkey::from_lines(&mut lines));
    };
    monkeys
}

fn _get_next_input_values(lines: &mut std::io::Lines<BufReader<File>>) -> String {
    lines.next().unwrap().unwrap().split(':').last().unwrap().to_owned()
}

fn _parse_worry_function(input: String) -> WorryFunction<'static> {
    let mut tokens = input.split_whitespace();
    let operand1 = tokens.next().unwrap().to_owned();
    let op = match tokens.next().unwrap() {
        "+" => std::ops::Add::add,
        "*" => std::ops::Mul::mul,
        _ => panic!(),
    };
    let operand2 = tokens.next().unwrap().to_owned();

    Box::new(move |old| op(
        operand1.parse::<u64>().unwrap_or(old),
        operand2.parse::<u64>().unwrap_or(old),
    ))
}
