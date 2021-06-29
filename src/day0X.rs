use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "./input/day0X.txt";

pub fn solve() {
    println!("Day 0Xa: {}", part1());
    println!("Day 0Xb: {}", part2());
}

fn part1() -> i32 {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}", line);
    }
    return -1;
}

fn part2() -> i32 {
    return -1;
}
