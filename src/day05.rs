use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day05.txt";

const ROWS: i32 = 128;
const COLUMNS: i32 = 8;

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let mut max: i32 = 0;
    let seats = parse();
    for seat in seats {
        if seat > max {
            max = seat;
        }
    }

    max
}

fn part2() -> i32 {
    let mut seats = parse();
    seats.sort_unstable();

    for (i, seat) in seats.iter().enumerate() {
        if i > 0 && seat - 1 != seats[i - 1] {
            return seat - 1;
        }
    }

    -1
}

fn parse() -> Vec<i32> {
    let mut seats: Vec<i32> = Vec::new();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();
        assert!(line.len() == 10);

        let mut low_row = 0;
        let mut high_row = ROWS - 1;
        let mut low_col = 0;
        let mut high_col = COLUMNS - 1;

        for (i, p) in line.iter().enumerate() {
            if i <= 6 {
                match p {
                    'F' => high_row -= (high_row - low_row + 1) / 2,
                    'B' => low_row += (high_row - low_row + 1) / 2,
                    _ => panic!(),
                };
            } else if i <= 9 {
                match p {
                    'L' => high_col -= (high_col - low_col + 1) / 2,
                    'R' => low_col += (high_col - low_col + 1) / 2,
                    _ => panic!(),
                };
            } else {
                panic!()
            }
        }
        assert!(low_row == high_row);
        assert!(low_col == high_col);
        seats.push(low_row * 8 + low_col);
    }

    seats
}
