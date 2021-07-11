use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day13.txt";

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(std::result::Result::unwrap).collect();
    assert_eq!(lines.len(), 2);
    let time: i32 = lines[0].parse().unwrap();
    let busses: Vec<i32> = lines[1]
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    let (bus, wait) = busses
        .iter()
        .map(|b| {
            (
                b,
                if time % b == 0 {
                    0
                } else {
                    (time / b) * b + b - time
                },
            )
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    bus * wait
}

fn part2() -> usize {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(std::result::Result::unwrap).collect();
    assert_eq!(lines.len(), 2);

    // Chinese Remainder Theorem: https://www.youtube.com/watch?v=zIFehsBHB8o
    // ni == bus_id
    // Tuple returned is (bi, bus_id)
    let busses: Vec<(usize, usize)> = lines[1]
        .split(',')
        .enumerate()
        .filter(|n| n.1 != "x")
        .map(|(i, s)| {
            let bus_id = s.parse().unwrap();
            ((bus_id - (i % bus_id)) % bus_id, bus_id)
        })
        .collect();

    let big_n = busses.iter().fold(1, |acc, (_, bus_id)| acc * bus_id);

    busses
        .iter()
        .map(|(bi, bus_id)| bi * big_n / bus_id * mod_inverse(big_n / bus_id, *bus_id))
        .sum::<usize>()
        % big_n
}

fn mod_inverse(n: usize, modulus: usize) -> usize {
    // Naive method: https://www.khanacademy.org/computing/computer-science/cryptography/modarithmetic/a/modular-inverses
    for i in 0..modulus {
        if (n * i) % modulus == 1 {
            return i;
        }
    }
    panic!();
}
