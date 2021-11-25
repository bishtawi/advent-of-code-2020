use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day18.txt";

#[derive(Debug, PartialEq, Eq)]
enum Op {
    None,
    Add,
    Mul,
}

struct Calc {
    op: Op,
    val: u64,
}

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> u64 {
    let mut sum = 0;
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        sum += calc(&line);
    }
    sum
}

fn part2() -> i32 {
    -1
}

fn calc(line: &str) -> u64 {
    let mut calcs: Vec<Calc> = vec![Calc {
        val: 0,
        op: Op::Add,
    }];

    for c in line.chars() {
        if let Some(v) = c.to_digit(10) {
            let latest = calcs.last_mut().unwrap();
            match latest.op {
                Op::Add => latest.val += u64::from(v),
                Op::Mul => latest.val *= u64::from(v),
                Op::None => panic!(),
            }
            latest.op = Op::None;
        } else if c == '*' {
            let latest = calcs.last_mut().unwrap();
            assert_eq!(latest.op, Op::None);
            latest.op = Op::Mul;
        } else if c == '+' {
            let latest = calcs.last_mut().unwrap();
            assert_eq!(latest.op, Op::None);
            latest.op = Op::Add;
        } else if c == '(' {
            calcs.push(Calc {
                val: 0,
                op: Op::Add,
            });
        } else if c == ')' {
            let Calc { val, op } = calcs.pop().unwrap();
            assert_eq!(op, Op::None);
            let latest = calcs.last_mut().unwrap();
            match latest.op {
                Op::Add => latest.val += val,
                Op::Mul => latest.val *= val,
                Op::None => panic!(),
            }
            latest.op = Op::None;
        } else if c == ' ' {
            continue;
        } else {
            panic!();
        }
    }

    assert_eq!(calcs.len(), 1);
    calcs.first().unwrap().val
}
