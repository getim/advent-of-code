use std::cmp::Ordering;
use std::fs::File;
use std::iter::{Product,Sum};
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::{Add,AddAssign,Mul};


#[derive(Debug,Eq,Clone,Copy)]
struct PacketResult {
    version_sum: usize,
    value: usize,
}

impl Add for PacketResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            version_sum: self.version_sum + other.version_sum,
            value: self.value + other.value,
        }
    }
}

impl AddAssign for PacketResult {
    fn add_assign(&mut self, other: Self) {
        self.version_sum += other.version_sum;
        self.value += other.value;
    }
}

impl<'a> Sum<&'a Self> for PacketResult {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item=&'a Self> {
        iter.fold(
            Self {
                value: 0,
                version_sum: 0,
            },
            |x, y| Self {
                value: x.value + y.value,
                version_sum: x.version_sum + y.version_sum,
            }
        )
    }

}

impl Mul for PacketResult {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            version_sum: self.version_sum + other.version_sum,
            value: self.value * other.value,
        }
    }
}

impl <'a> Product<&'a Self> for PacketResult {
    fn product<I>(iter: I) -> Self where I: Iterator<Item=&'a Self> {
        iter.fold(
            Self {
                value: 1,
                version_sum: 0,
            },
            |x, y| Self {
                value: x.value * y.value,
                version_sum: x.version_sum + y.version_sum,
            }
        )
    }
}

impl Ord for PacketResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for PacketResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketResult {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}


fn main() {
    let file = File::open("input").unwrap();
    let hex_message = BufReader::new(file).lines().next().unwrap().unwrap();
    let bits = hex_message.chars()
                              .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
                              .collect::<Vec<_>>()
                              .join("");

    println!("Result 1: {}", solve1(&mut bits.chars()));
    println!("Result 2: {}", solve2(&mut bits.chars()));
}

fn solve1<I>(bits: &mut I) -> usize
    where I: Iterator<Item=char> + std::fmt::Debug
{
    read_full_message(bits).version_sum
}

fn solve2<I>(bits: &mut I) -> usize
    where I: Iterator<Item=char> + std::fmt::Debug
{
    read_full_message(bits).value
}

fn read_full_message<I>(bits: &mut I) -> PacketResult
    where I: Iterator<Item=char> + std::fmt::Debug
{
    read_all_packets(bits).pop().unwrap()
}

fn read_all_packets<I>(bits: &mut I) -> Vec<PacketResult> where I: Iterator<Item=char> {
    let mut packets = Vec::new();
    loop {
        let version_string: String = bits.take(3).collect();
        let version = match usize::from_str_radix(&version_string, 2) {
            Ok(v) => v,
            _ => break
        };
        let type_id: String = bits.take(3).collect();
        let loop_result = if type_id == "100" {
            read_literal(bits, version)
        } else {
            read_operator(bits, &type_id, version)
        };
        // Stop reading packets when the individual readers failed to read a correct packet.
        match loop_result {
            Some(r) => packets.push(r),
            None => break,
        }
    }
    packets
}

fn read_n_packets<I>(bits: &mut I, n: usize) -> Vec<PacketResult> where I: Iterator<Item=char> {
    let mut packets = Vec::new();
    for _ in 0..n {
        let version_string: String = bits.take(3).collect();
        let version = usize::from_str_radix(&version_string, 2).unwrap();
        let type_id: String = bits.take(3).collect();
        // Safely unwrap as we assume we can always read n correct packets.
        if type_id == "100" {
            packets.push(read_literal(bits, version).unwrap());
        } else {
            packets.push(read_operator(bits, &type_id, version).unwrap());
        }
    }
    packets
}

fn read_literal<I>(bits: &mut I, version: usize) -> Option<PacketResult>
    where I: Iterator<Item=char>
{
    let mut literal: Vec<String> = Vec::new();
    loop {
        let keep_reading = bits.next()? == '1';
        literal.push(bits.take(4).collect());
        if !keep_reading {
            break;
        }
    }
    Some(PacketResult {
        version_sum: version,
        value: usize::from_str_radix(&literal.join(""), 2).unwrap(),
    })
}

fn read_operator<I>(bits: &mut I, type_id: &str, version: usize) -> Option<PacketResult>
    where I: Iterator<Item=char>
{
    let subpackets = if bits.next()? == '0' {
        let subpacket_length =
            usize::from_str_radix(&bits.take(15).collect::<String>(), 2).ok()?;
        // We can't just pass the Take as that will make the compiler hit infinite recursion
        // while evaluating the generics Take<Take<Take<...>>>
        read_all_packets(&mut bits.take(subpacket_length).collect::<String>().chars())
    } else {
        let subpacket_count =
            usize::from_str_radix(&bits.take(11).collect::<String>(), 2).ok()?;
        read_n_packets(bits, subpacket_count)
    };

    let mut result: Option<PacketResult> = match type_id {
        "000" => Some(subpackets.iter().sum()),
        "001" => Some(subpackets.iter().product()),
        "010" => Some(*subpackets.iter().min().unwrap()),
        "011" => Some(*subpackets.iter().max().unwrap()),
        _ => {
            let value = match type_id {
                "101" => Some(if subpackets[0] > subpackets[1] { 1 } else { 0 }),
                "110" => Some(if subpackets[0] < subpackets[1] { 1 } else { 0 }),
                "111" => Some(if subpackets[0] == subpackets[1] { 1 } else { 0 }),
                _ => None,
            };
            value.map(|v| PacketResult {
                value: v,
                version_sum: subpackets[0].version_sum + subpackets[1].version_sum,
            })
        }
    };

    if let Some(ref mut r) = result {
        r.version_sum += version;
    }
    result
}
