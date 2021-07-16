use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day16.txt";

#[derive(Debug)]
struct Input {
    rules: HashMap<String, Vec<(u64, u64)>>,
    my_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            rules: HashMap::new(),
            my_ticket: Vec::new(),
            nearby_tickets: Vec::new(),
        }
    }
}

pub fn solve() {
    let input = parse();
    println!("{}: {} | {}", INPUT, part1(&input), part2(&input));
}

fn part1(input: &Input) -> u64 {
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&v| {
            !input
                .rules
                .values()
                .any(|r| r.iter().any(|(min, max)| min <= v && v <= max))
        })
        .sum()
}

fn part2(input: &Input) -> u64 {
    let valid_tickets: Vec<&Vec<u64>> = input
        .nearby_tickets
        .iter()
        .filter(|&t| {
            t.iter().all(|v| {
                input
                    .rules
                    .values()
                    .any(|r| r.iter().any(|(min, max)| min <= v && v <= max))
            })
        })
        .collect();

    let mut possibilities = vec![input.rules.keys().collect::<HashSet<_>>(); input.my_ticket.len()];

    for ticket in valid_tickets {
        for (i, v) in ticket.iter().enumerate() {
            possibilities[i]
                .retain(|&s| input.rules[s].iter().any(|(min, max)| min <= v && v <= max));
        }
    }

    let mut field = *possibilities
        .iter()
        .find(|&p| p.len() == 1)
        .unwrap()
        .iter()
        .next()
        .unwrap();
    loop {
        let mut next_field = field;
        for poss in &mut possibilities {
            match poss.len() {
                0 => panic!(),
                1 => continue,
                2 => {
                    poss.remove(field);
                    assert_eq!(next_field, field);
                    next_field = poss.iter().next().unwrap();
                }
                _ => {
                    poss.remove(field);
                }
            }
        }
        if next_field == field {
            break;
        }
        field = next_field;
    }

    possibilities.iter().enumerate().fold(1, |acc, (i, p)| {
        assert_eq!(p.len(), 1);
        let field = *p.iter().next().unwrap();
        if field.starts_with("departure") {
            acc * input.my_ticket[i]
        } else {
            acc
        }
    })
}

fn parse() -> Input {
    let mut input = Input::default();
    let mut phase = 0;

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        match line.as_str() {
            "" => continue,
            "your ticket:" => {
                assert_eq!(phase, 0);
                phase = 1;
            }
            "nearby tickets:" => {
                assert_eq!(phase, 1);
                phase = 2;
            }
            _ => match phase {
                0 => {
                    let segs: Vec<&str> = line.split(": ").collect();
                    assert_eq!(segs.len(), 2);
                    let ranges: Vec<&str> = segs[1].split(" or ").collect();
                    input.rules.insert(
                        segs[0].to_string(),
                        ranges
                            .iter()
                            .map(|&r| {
                                let range: Vec<&str> = r.split('-').collect();
                                assert_eq!(range.len(), 2);
                                let min = range[0].parse().unwrap();
                                let max = range[1].parse().unwrap();
                                assert!(min < max);
                                (min, max)
                            })
                            .collect(),
                    );
                }
                1 => {
                    assert_eq!(input.my_ticket.len(), 0);
                    input.my_ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
                }
                2 => {
                    input
                        .nearby_tickets
                        .push(line.split(',').map(|s| s.parse().unwrap()).collect());
                }
                _ => panic!(),
            },
        }
    }

    input
}
