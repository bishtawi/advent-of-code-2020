use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day12.txt";

enum Action {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

enum Direction {
    North,
    East,
    South,
    West,
}

pub fn solve() {
    let actions = parse();
    println!("{}: {} | {}", INPUT, part1(&actions), part2(&actions));
}

fn part1(actions: &[Action]) -> i32 {
    let mut cur_dir = Direction::East;
    let mut east_pos: i32 = 0;
    let mut north_pos: i32 = 0;

    for action in actions {
        match action {
            Action::North(val) => north_pos += val,
            Action::South(val) => north_pos -= val,
            Action::East(val) => east_pos += val,
            Action::West(val) => east_pos -= val,
            Action::Forward(val) => match cur_dir {
                Direction::North => north_pos += val,
                Direction::South => north_pos -= val,
                Direction::East => east_pos += val,
                Direction::West => east_pos -= val,
            },
            Action::Left(deg) => {
                cur_dir = turn_left(cur_dir, *deg);
            }
            Action::Right(deg) => {
                cur_dir = turn_right(cur_dir, *deg);
            }
        }
    }

    east_pos.abs() + north_pos.abs()
}

fn part2(actions: &[Action]) -> i32 {
    let mut east_pos: i32 = 0;
    let mut north_pos: i32 = 0;
    let mut waypoint_east: i32 = 10;
    let mut waypoint_north: i32 = 1;

    for action in actions {
        match action {
            Action::North(val) => waypoint_north += val,
            Action::South(val) => waypoint_north -= val,
            Action::East(val) => waypoint_east += val,
            Action::West(val) => waypoint_east -= val,
            Action::Forward(val) => {
                east_pos += val * waypoint_east;
                north_pos += val * waypoint_north;
            }
            Action::Left(deg) => match (deg / 90) % 4 {
                0 => (),
                1 => {
                    let temp = waypoint_north;
                    waypoint_north = waypoint_east;
                    waypoint_east = -temp;
                }
                2 => {
                    waypoint_east = -waypoint_east;
                    waypoint_north = -waypoint_north;
                }
                3 => {
                    let temp = waypoint_east;
                    waypoint_east = waypoint_north;
                    waypoint_north = -temp;
                }
                _ => panic!(),
            },
            Action::Right(deg) => match (deg / 90) % 4 {
                0 => (),
                1 => {
                    let temp = waypoint_east;
                    waypoint_east = waypoint_north;
                    waypoint_north = -temp;
                }
                2 => {
                    waypoint_east = -waypoint_east;
                    waypoint_north = -waypoint_north;
                }
                3 => {
                    let temp = waypoint_north;
                    waypoint_north = waypoint_east;
                    waypoint_east = -temp;
                }
                _ => panic!(),
            },
        }
    }

    east_pos.abs() + north_pos.abs()
}

fn parse() -> Vec<Action> {
    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (action, val) = line.split_at(1);
            let val: i32 = val.parse().unwrap();
            match action {
                "N" => Action::North(val),
                "S" => Action::South(val),
                "E" => Action::East(val),
                "W" => Action::West(val),
                "L" => Action::Left(val),
                "R" => Action::Right(val),
                "F" => Action::Forward(val),
                _ => panic!(),
            }
        })
        .collect()
}

fn turn_left(mut direction: Direction, degrees: i32) -> Direction {
    let mut turns = (degrees / 90) % 4;
    while turns > 0 {
        direction = match direction {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        turns -= 1;
    }
    direction
}

fn turn_right(mut direction: Direction, degrees: i32) -> Direction {
    let mut turns = (degrees / 90) % 4;
    while turns > 0 {
        direction = match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        turns -= 1;
    }
    direction
}
