use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day10.txt";

pub fn solve() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let mut jolts: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    jolts.sort_unstable();

    println!("{}: {} | {}", INPUT, part1(&jolts), part2(&jolts));
}

fn part1(jolts: &[i32]) -> i32 {
    let mut curr_jolt: i32 = 0;
    let mut one_jolt: i32 = 0;
    let mut three_jolt: i32 = 1;

    for jolt in jolts {
        match jolt - curr_jolt {
            1 => one_jolt += 1,
            2 => (),
            3 => three_jolt += 1,
            _ => panic!(),
        }

        curr_jolt = *jolt;
    }

    one_jolt * three_jolt
}

fn part2(jolts: &[i32]) -> u64 {
    let mut mem: HashMap<i32, u64> = HashMap::new();
    mem.insert(jolts.last().unwrap() + 3, 1);
    for jolt in jolts.iter().rev() {
        let total: u64 = [1, 2, 3].iter().fold(0, |a, d| {
            if let Some(val) = mem.get(&(jolt + d)) {
                a + val
            } else {
                a
            }
        });
        mem.insert(*jolt, total);
    }

    let total: u64 = [1, 2, 3]
        .iter()
        .fold(0, |a, d| if let Some(v) = mem.get(&d) { a + v } else { a });

    total

    // Recursion is too slow for the puzzle input
    // return part2_recursive(jolts, 0, 0);
}

fn _part2_recursive(jolts: &[i32], start_index: usize, curr_jolt: i32) -> u64 {
    if start_index >= jolts.len() {
        return 1;
    }

    let mut total: u64 = 0;
    for i in start_index..jolts.len() {
        if jolts[i] - curr_jolt > 3 {
            break;
        }
        total += _part2_recursive(jolts, i + 1, jolts[i]);
    }

    total
}
