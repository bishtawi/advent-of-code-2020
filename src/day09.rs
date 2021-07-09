use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day09.txt";

const PREAMBLE: usize = 25;

pub fn solve() {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let nums: Vec<i64> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let p1 = part1(&nums);
    println!("{}: {} | {}", INPUT, p1, part2(&nums, p1));
}

fn part1(nums: &[i64]) -> i64 {
    for (i, sum) in nums.iter().enumerate() {
        if i < PREAMBLE {
            continue;
        }

        let mut flag = false;
        let mut seen: HashSet<i64> = HashSet::new();
        for num in nums.iter().take(i).skip(i - PREAMBLE) {
            let other = sum - num;
            if seen.contains(&other) {
                flag = true;
                break;
            }
            seen.insert(*num);
        }

        if !flag {
            return *sum;
        }
    }

    0
}

fn part2(nums: &[i64], sum: i64) -> i64 {
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut total: i64 = nums[start] + nums[end];
    for (i, num) in nums.iter().enumerate() {
        if i <= 1 {
            continue;
        }
        if total == sum {
            break;
        }
        total += num;
        end += 1;
        while total > sum {
            total -= nums[start];
            start += 1;
        }
    }

    let min = nums[start..=end]
        .iter()
        .reduce(|a, b| if a > b { b } else { a })
        .unwrap();
    let max = nums[start..=end]
        .iter()
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap();

    min + max
}
