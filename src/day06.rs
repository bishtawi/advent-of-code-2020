use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day06.txt";

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> u64 {
    let mut total: u64 = 0;
    let mut group: HashSet<char> = HashSet::new();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();

        if line.is_empty() {
            total += group.len() as u64;
            group = HashSet::new();
            continue;
        }

        for character in line {
            group.insert(character);
        }
    }
    total += group.len() as u64;

    total
}

fn part2() -> u64 {
    let mut total: u64 = 0;
    let mut group: HashSet<char> = HashSet::new();
    let mut flag = true;

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();

        if line.is_empty() {
            total += group.len() as u64;
            flag = true;
            continue;
        }

        let mut person: HashSet<char> = HashSet::new();
        for character in line {
            person.insert(character);
        }

        if flag {
            flag = false;
            group = person;
        } else {
            group = group.intersection(&person).copied().collect();
        }
    }
    total += group.len() as u64;

    total
}
