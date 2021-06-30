use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day01.txt";
const SUM: i32 = 2020;

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let num: i32 = line.unwrap().parse().unwrap();
        let opposite = SUM - num;

        if set.contains(&opposite) {
            return num * opposite;
        }

        set.insert(num);
    }

    return -1;
}

fn part2() -> i32 {
    let mut entries: HashMap<i32, i32> = HashMap::new();
    let mut list: Vec<i32> = Vec::new();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let num: i32 = line.unwrap().parse().unwrap();

        list.push(num);

        if entries.contains_key(&num) {
            *entries.get_mut(&num).unwrap() += 1;
        } else {
            entries.insert(num, 1);
        }
    }

    for (i, x) in list.iter().enumerate() {
        for (j, y) in list.iter().enumerate() {
            if i == j {
                continue;
            }

            let other = SUM - x - y;
            if entries.contains_key(&other) {
                let mut count = entries[&other];
                if other == *x {
                    count -= 1;
                }
                if other == *y {
                    count -= 1;
                }
                if count > 0 {
                    return x * y * other;
                }
            }
        }
    }

    return -1;
}
