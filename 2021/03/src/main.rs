use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let file = File::open("input").unwrap();
    let numbers: Vec<Vec<bool>> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '1').collect())
        .collect();

    println!("Result 1: {}", solve1(&numbers));
    println!("Result 1: {}", solve2(&numbers));
}

fn solve1(numbers: &[Vec<bool>]) -> i32 {
    let mut counts = vec![0; numbers[0].len()];
    for number in numbers {
        for (i, bit) in number.iter().enumerate() {
            if *bit {
                counts[i] += 1;
            }
        }
    }

    let majority_len = numbers.len() / 2;
    let gamma: Vec<bool> = counts.iter().map(|c| *c > majority_len).collect();
    let epsilon: Vec<bool> = gamma.iter().map(|c| !c).collect();

    to_number(&gamma) * to_number(&epsilon)
}

fn solve2(numbers: &Vec<Vec<bool>>) -> i32 {
    let mut gamma_candidates = numbers.clone();
    eliminate(&mut gamma_candidates, false);
    let mut epsilon_candidates = numbers.clone();
    eliminate(&mut epsilon_candidates, true);

    to_number(&gamma_candidates[0]) * to_number(&epsilon_candidates[0])
}

fn to_number(bits: &[bool]) -> i32
{
    i32::from_str_radix(bits.iter().map(|b| if *b { '1' } else { '0' }).collect::<String>().as_str(), 2).unwrap()
}

fn eliminate(numbers: &mut Vec<Vec<bool>>, invert: bool) {
    let mut bit_pos = 0;
    while numbers.len() > 1 {
        let bit_count = numbers.iter().filter(|n| n[bit_pos]).count();
        let mut winner_bit = bit_count as f64 >= numbers.len() as f64 / 2f64;
        if invert {
            winner_bit = !winner_bit
        }
        numbers.retain(|n| n[bit_pos] == winner_bit);
        bit_pos += 1;
    };
}
