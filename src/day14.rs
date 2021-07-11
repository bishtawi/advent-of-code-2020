use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day14.txt";

enum Instruction {
    Mask(String),
    Mem(u64, u64), // addr, val
}

pub fn solve() {
    let instructions = parse();
    println!(
        "{}: {} | {}",
        INPUT,
        part1(&instructions),
        part2(&instructions)
    );
}

fn part1(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut cur_and = u64::MAX;
    let mut cur_or = u64::MIN;
    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                let (and, or) = build_bit_masks(&mask);
                cur_and = and;
                cur_or = or;
            }
            Instruction::Mem(mem, val) => {
                memory.insert(*mem, val & cur_and | cur_or);
            }
        }
    }

    memory.iter().map(|(_, val)| val).sum::<u64>()
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut cur_mask = &String::from("");
    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => {
                cur_mask = mask;
            }
            Instruction::Mem(mem, val) => {
                let mut addrs: Vec<u64> = vec![*mem];
                for (i, c) in cur_mask.chars().enumerate() {
                    let bit = 1 << (36 - i - 1);
                    match c {
                        'X' => addrs = addrs.iter().flat_map(|a| [a | bit, a & !bit]).collect(),
                        '0' => (),
                        '1' => addrs = addrs.iter().map(|a| a | bit).collect(),
                        _ => panic!(),
                    }
                }
                for addr in addrs {
                    memory.insert(addr, *val);
                }
            }
        }
    }

    memory.iter().map(|(_, val)| val).sum::<u64>()
}

fn parse() -> Vec<Instruction> {
    lazy_static! {
        static ref MEM: Regex = Regex::new(r"^mem\[(?P<addr>\d+)\]$").unwrap();
    }

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let parts: Vec<&str> = line.split(" = ").collect();
            assert_eq!(parts.len(), 2);
            if parts[0] == "mask" {
                Instruction::Mask(parts[1].to_string())
            } else {
                match MEM.captures(parts[0]) {
                    Some(g) => {
                        Instruction::Mem(g["addr"].parse().unwrap(), parts[1].parse().unwrap())
                    }
                    _ => panic!(),
                }
            }
        })
        .collect()
}

fn build_bit_masks(orig: &str) -> (u64, u64) {
    let mut and = u64::MIN;
    let mut or = u64::MIN;

    let len = orig.len();
    for (i, c) in orig.chars().enumerate() {
        let bit = 1 << (len - i - 1);
        match c {
            'X' => (),
            '0' => and |= bit,
            '1' => or |= bit,
            _ => panic!(),
        }
    }

    (!and, or)
}
