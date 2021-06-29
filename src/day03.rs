use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "./input/day03.txt";

pub fn solve() {
    println!("Day 03a: {}", part1());
    println!("Day 03b: {}", part2());
}

fn part1() -> i32 {
    let mut pos = 0;
    let mut len = 0;
    let mut trees = 0;
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            len = line.len();
        } else {
            assert!(len == line.len());
        }
        if line.chars().nth(pos).unwrap() == '#' {
            trees += 1;
        }
        pos = (pos + 3) % len;
    }
    return trees;
}

fn part2() -> i64 {
    let mut len = 0;
    let mut trees: Vec<Vec<char>> = Vec::new();

    let mut total: i64 = 0;
    let mut pos = 0;

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();
        if i == 0 {
            len = line.len();
        } else {
            assert!(len == line.len());
        }

        if i % 2 == 0 {
            if line[pos] == '#' {
                total += 1;
            }
            pos = (pos + 1) % len;
        }

        trees.push(line);
    }

    for jump in [1, 3, 5, 7] {
        let mut count = 0;
        pos = 0;

        for line in &trees {
            if line[pos] == '#' {
                count += 1;
            }

            pos = (pos + jump) % len;
        }

        total *= count;
    }

    return total;
}
