use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day07.txt";

const MY_BAG: &str = "shiny gold";

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> usize {
    let bags = parse();
    let mut searched: HashSet<String> = HashSet::new();
    let mut to_search: Vec<&str> = [MY_BAG].to_vec();

    while !to_search.is_empty() {
        let target_bag = to_search.pop().unwrap();
        for (curr_bag, inner_bags) in bags.iter() {
            if inner_bags.contains_key(target_bag) && !searched.contains(curr_bag) {
                to_search.push(curr_bag);
                searched.insert(curr_bag.to_string());
            }
        }
    }

    searched.len()
}

fn part2() -> i32 {
    let bags = parse();

    bag_count(&bags, MY_BAG) - 1
}

fn bag_count(bags: &HashMap<String, HashMap<String, i32>>, bag: &str) -> i32 {
    let inner_bags = &bags[bag];
    let mut total: i32 = 1;
    for (inner_bag, count) in inner_bags.iter() {
        total += count * bag_count(bags, inner_bag);
    }

    total
}

fn parse() -> HashMap<String, HashMap<String, i32>> {
    lazy_static! {
        static ref BAG: Regex =
            Regex::new(r"^ ?(?P<count>\d+) (?P<color>[a-z ]+) bags?\.?$").unwrap();
    }

    let mut bags: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(" contain ").collect();
        assert!(split.len() == 2);

        let bag_color = split[0].strip_suffix(" bags").unwrap().to_string();
        let mut inner_bags: HashMap<String, i32> = HashMap::new();
        if split[1] != "no other bags." {
            let bag_descs: Vec<&str> = split[1].split(',').collect();
            for bag_desc in bag_descs {
                match BAG.captures(bag_desc) {
                    Some(g) => {
                        let count = g["count"].parse::<i32>().unwrap();
                        let inner_bag_color = &g["color"];
                        inner_bags.insert(inner_bag_color.to_string(), count);
                    }
                    _ => panic!("{}", bag_desc),
                }
            }
        }
        bags.insert(bag_color, inner_bags);
    }

    bags
}
