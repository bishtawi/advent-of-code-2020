use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day15.txt";

pub fn solve() {
    let starters = parse();
    println!("{}: {} | {}", INPUT, part1(&starters), part2(&starters));
}

fn part1(starters: &[usize]) -> usize {
    exec(starters, 2020)
}

fn part2(starters: &[usize]) -> usize {
    exec(starters, 30_000_000)
}

fn parse() -> Vec<usize> {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .flat_map(|l| {
            l.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn exec(starters: &[usize], turns: usize) -> usize {
    let mut history: HashMap<usize, usize> = HashMap::new();
    let mut num = 0;
    for (i, val) in starters.iter().enumerate() {
        if i == starters.len() - 1 {
            num = *val;
        } else {
            history.insert(*val, i + 1);
        }
    }
    for i in starters.len()..turns {
        let next_num = match history.get(&num) {
            Some(j) => i - j,
            None => 0,
        };
        history.insert(num, i);
        num = next_num;
    }
    num
}
