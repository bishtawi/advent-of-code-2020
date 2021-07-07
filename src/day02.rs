use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day02.txt";

struct Entry {
    lower_bound: u64,
    upper_bound: u64,
    letter: char,
    password: String,
}

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let entries: Vec<Entry> = parse();

    let mut valid: i32 = 0;
    for entry in entries {
        let mut count: u64 = 0;
        for character in entry.password.chars() {
            if character == entry.letter {
                count += 1;
            }
        }
        if count <= entry.upper_bound && count >= entry.lower_bound {
            valid += 1;
        }
    }

    valid
}

#[allow(clippy::nonminimal_bool)]
fn part2() -> i32 {
    let entries: Vec<Entry> = parse();

    let mut valid: i32 = 0;

    for entry in entries {
        let characters: Vec<char> = entry.password.chars().collect();
        if characters.len() < entry.lower_bound as usize {
            continue;
        }
        let flag = characters[entry.lower_bound as usize - 1] == entry.letter;

        if (characters.len() < entry.upper_bound as usize && flag)
            || (characters[entry.upper_bound as usize - 1] != entry.letter && flag)
            || (characters[entry.upper_bound as usize - 1] == entry.letter && !flag)
        {
            valid += 1;
        }
    }

    valid
}

fn parse() -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        assert!(parts.len() == 3);
        let ranges: Vec<&str> = parts[0].split('-').collect();
        assert!(ranges.len() == 2);
        let lower_bound: u64 = ranges[0].parse().unwrap();
        let upper_bound: u64 = ranges[1].parse().unwrap();
        assert!(lower_bound < upper_bound);
        assert!(parts[1].len() == 2);
        let letter = parts[1].chars().next().unwrap();
        let password = parts[2].to_string();
        entries.push(Entry {
            lower_bound,
            upper_bound,
            letter,
            password,
        });
    }

    entries
}
