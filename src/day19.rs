use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day19.txt";

#[derive(Debug, PartialEq, Eq)]
enum Pattern {
    Concrete(HashSet<String>),
    Reference(Vec<Vec<u32>>),
}

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> usize {
    let mut rules: HashMap<u32, Pattern> = HashMap::new();
    let mut tests: Vec<String> = Vec::new();
    let mut parse_tests = false;

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            assert!(!parse_tests);
            parse_tests = true;
            continue;
        }

        if parse_tests {
            tests.push(line);
            continue;
        }

        let parts: Vec<_> = line.split(": ").collect();
        assert_eq!(parts.len(), 2);
        let key: u32 = parts[0].parse().unwrap();
        let rule = parts[1];

        if rule.starts_with('"') && rule.ends_with('"') {
            let mut val = HashSet::new();
            val.insert(rule[1..rule.len() - 1].to_string());
            rules.insert(key, Pattern::Concrete(val));
            continue;
        }

        rules.insert(
            key,
            Pattern::Reference(
                rule.split(" | ")
                    .map(|s| s.split(' ').map(|i| i.parse().unwrap()).collect())
                    .collect(),
            ),
        );
    }

    loop {
        let mut done = true;
        let mut new_patterns: HashMap<u32, Pattern> = HashMap::new();
        for (key, pattern) in &rules {
            match pattern {
                Pattern::Concrete(_) => continue,
                Pattern::Reference(refs) => {
                    let ready = refs
                        .iter()
                        .flatten()
                        .all(|k| matches!(&rules[k], Pattern::Concrete(_)));
                    if !ready {
                        done = false;
                        continue;
                    }

                    new_patterns.insert(
                        *key,
                        Pattern::Concrete(
                            refs.iter()
                                .flat_map(|r| {
                                    r.iter().fold(HashSet::new(), |acc: HashSet<String>, k| {
                                        if let Pattern::Concrete(p) = &rules[k] {
                                            if acc.is_empty() {
                                                p.iter().cloned().collect()
                                            } else {
                                                acc.iter()
                                                    .flat_map(|a| {
                                                        let combos: HashSet<String> = p
                                                            .iter()
                                                            .map(|b| {
                                                                let mut combined = a.to_string();
                                                                combined.push_str(b);
                                                                combined
                                                            })
                                                            .collect();
                                                        combos
                                                    })
                                                    .collect()
                                            }
                                        } else {
                                            panic!();
                                        }
                                    })
                                })
                                .collect(),
                        ),
                    );
                }
            }
        }

        for (key, pattern) in new_patterns {
            rules.insert(key, pattern);
        }

        if done {
            break;
        }
    }

    if let Pattern::Concrete(r) = &rules[&0] {
        tests.iter().filter(|&t| r.contains(t)).count()
    } else {
        panic!()
    }
}

fn part2() -> i32 {
    -1
}
