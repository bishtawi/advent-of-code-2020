use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day1X.txt";

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}", line);
    }
    -1
}

fn part2() -> i32 {
    -1
}
