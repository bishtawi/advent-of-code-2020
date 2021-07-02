use std::collections::HashSet;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day08.txt";

#[derive(Debug)]
struct Instruction {
    operation: String,
    argument: i32,
}

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let instructions = parse();
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        assert!(i >= 0);
        let instruction = &instructions[i as usize];
        if visited.contains(&i) {
            break;
        } else {
            visited.insert(i);
        }
        match instruction.operation.as_str() {
            "acc" => {
                acc += instruction.argument;
                i += 1;
            }
            "jmp" => i += instruction.argument,
            "nop" => i += 1,
            _ => panic!(),
        }
    }

    return acc;
}

fn part2() -> i32 {
    let instructions = parse();
    let mut i: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut possibilities: Vec<i32> = Vec::new();

    loop {
        assert!(i >= 0);
        let instruction = &instructions[i as usize];
        if visited.contains(&i) {
            break;
        } else {
            visited.insert(i);
        }
        match instruction.operation.as_str() {
            "acc" => {
                i += 1;
            }
            "jmp" => {
                if instruction.argument != 1 {
                    possibilities.push(i);
                }
                i += instruction.argument;
            }
            "nop" => {
                if instruction.argument != 0 {
                    possibilities.push(i);
                }
                i += 1;
            }
            _ => panic!(),
        }
    }

    for pos in possibilities {
        let (acc, done) = execute(&instructions, pos);
        if done {
            return acc;
        }
    }

    return -1;
}

fn execute(instructions: &Vec<Instruction>, swap: i32) -> (i32, bool) {
    let mut acc: i32 = 0;
    let mut i: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        assert!(i >= 0);
        if i >= instructions.len().try_into().unwrap() {
            return (acc, true);
        }
        let instruction = &instructions[i as usize];
        if visited.contains(&i) {
            return (acc, false);
        } else {
            visited.insert(i);
        }
        let mut op = instruction.operation.as_str();
        if swap == i {
            op = match op {
                "acc" => "acc",
                "jmp" => "nop",
                "nop" => "jmp",
                _ => panic!(),
            }
        }
        match op {
            "acc" => {
                acc += instruction.argument;
                i += 1;
            }
            "jmp" => {
                i += instruction.argument;
            }
            "nop" => {
                i += 1;
            }
            _ => panic!(),
        }
    }
}

fn parse() -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" ").collect();
        assert!(parts.len() == 2);
        instructions.push(Instruction {
            operation: parts[0].to_string(),
            argument: parts[1].parse().unwrap(),
        })
    }

    return instructions;
}
