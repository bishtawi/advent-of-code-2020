use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day11.txt";

#[derive(Clone, PartialEq, Eq)]
enum Spot {
    Taken,
    Available,
    Floor,
}

pub fn solve() {
    let area = parse();
    println!("{}: {} | {}", INPUT, part1(area.clone()), part2(area));
}

fn part1(area: Vec<Vec<Spot>>) -> i32 {
    iterate(area, true)
}

fn part2(area: Vec<Vec<Spot>>) -> i32 {
    iterate(area, false)
}

fn parse() -> Vec<Vec<Spot>> {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spot::Floor,
                    'L' => Spot::Available,
                    '#' => Spot::Taken,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

fn iterate(mut curr_round: Vec<Vec<Spot>>, is_part1: bool) -> i32 {
    let mut next_round = curr_round.clone();

    loop {
        for (i, row) in curr_round.iter().enumerate() {
            for (j, spot) in row.iter().enumerate() {
                match spot {
                    Spot::Available => {
                        if will_take_seat(&curr_round, i, j, is_part1) {
                            next_round[i][j] = Spot::Taken;
                        }
                    }
                    Spot::Taken => {
                        if will_leave_seat(&curr_round, i, j, is_part1) {
                            next_round[i][j] = Spot::Available;
                        }
                    }
                    _ => (),
                }
            }
        }

        if curr_round == next_round {
            break;
        } else {
            curr_round = next_round.clone();
        }
    }

    curr_round
        .iter()
        .flatten()
        .fold(0, |a, s| if s == &Spot::Taken { a + 1 } else { a })
}

fn will_take_seat(spots: &[Vec<Spot>], row: usize, col: usize, is_part1: bool) -> bool {
    if spots[row][col] != Spot::Available {
        panic!();
    }

    adjacent_spots(spots, row, col, is_part1)
        .iter()
        .fold(true, |acc, (i, j)| acc && !seat_taken(spots, *i, *j))
}

fn will_leave_seat(spots: &[Vec<Spot>], row: usize, col: usize, is_part1: bool) -> bool {
    if spots[row][col] != Spot::Taken {
        panic!();
    }

    let count = adjacent_spots(spots, row, col, is_part1)
        .iter()
        .fold(0, |acc, (i, j)| {
            if seat_taken(spots, *i, *j) {
                acc + 1
            } else {
                acc
            }
        });

    count >= if is_part1 { 4 } else { 5 }
}

fn seat_taken(spots: &[Vec<Spot>], i: usize, j: usize) -> bool {
    if let Some(row) = spots.get(i) {
        if let Some(spot) = row.get(j) {
            return spot == &Spot::Taken;
        }
    }

    false
}

fn adjacent_spots(
    spots: &[Vec<Spot>],
    row: usize,
    col: usize,
    is_part1: bool,
) -> Vec<(usize, usize)> {
    if is_part1 {
        // Lacking proper bounds checking here. But that is okay as function seat_taken has bounds checks

        let mut adj = vec![(row, col + 1), (row + 1, col), (row + 1, col + 1)];

        if row > 0 {
            adj.push((row - 1, col));
            adj.push((row - 1, col + 1));
        }

        if col > 0 {
            adj.push((row, col - 1));
            adj.push((row + 1, col - 1));
            if row > 0 {
                adj.push((row - 1, col - 1));
            }
        }

        adj
    } else {
        let mut adj: Vec<(usize, usize)> = Vec::new();

        // Theres probably a better way to do this...

        // Row decreasing
        {
            let mut i = row;
            while i > 0 {
                i -= 1;
                if spots[i][col] != Spot::Floor {
                    adj.push((i, col));
                    break;
                }
            }
        }

        // Row increasing
        {
            let mut i = row;
            while i < spots.len() - 1 {
                i += 1;
                if spots[i][col] != Spot::Floor {
                    adj.push((i, col));
                    break;
                }
            }
        }

        // Column decreasing
        {
            let mut j = col;
            while j > 0 {
                j -= 1;
                if spots[row][j] != Spot::Floor {
                    adj.push((row, j));
                    break;
                }
            }
        }

        // Column increasing
        {
            let mut j = col;
            while j < spots[row].len() - 1 {
                j += 1;
                if spots[row][j] != Spot::Floor {
                    adj.push((row, j));
                    break;
                }
            }
        }

        // Row and column decreasing
        {
            let mut i = row;
            let mut j = col;
            while i > 0 && j > 0 {
                i -= 1;
                j -= 1;
                if spots[i][j] != Spot::Floor {
                    adj.push((i, j));
                    break;
                }
            }
        }

        // Row and column increasing
        {
            let mut i = row;
            let mut j = col;
            while i < spots.len() - 1 && j < spots[row].len() - 1 {
                i += 1;
                j += 1;
                if spots[i][j] != Spot::Floor {
                    adj.push((i, j));
                    break;
                }
            }
        }

        // Row decreasing
        // Column increasing
        {
            let mut i = row;
            let mut j = col;
            while i > 0 && j < spots[row].len() - 1 {
                i -= 1;
                j += 1;
                if spots[i][j] != Spot::Floor {
                    adj.push((i, j));
                    break;
                }
            }
        }

        // Row increasing
        // Column decreasing
        {
            let mut i = row;
            let mut j = col;
            while i < spots.len() - 1 && j > 0 {
                i += 1;
                j -= 1;
                if spots[i][j] != Spot::Floor {
                    adj.push((i, j));
                    break;
                }
            }
        }

        adj
    }
}
