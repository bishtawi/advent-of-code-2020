use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT: &str = "input/day17.txt";

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

// Could probably use some refactoring (abstracting out the dimensions) to minimize code duplication

fn part1() -> i32 {
    let mut poc_dim: HashMap<[i32; 3], bool> = HashMap::new();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, char) in line.chars().enumerate() {
            let active = match char {
                '.' => false,
                '#' => true,
                _ => panic!(),
            };
            let coord = [i.try_into().unwrap(), j.try_into().unwrap(), 0];
            poc_dim.insert(coord, active);
            if active {
                for neighbor in get_3d_neighbors(&coord) {
                    poc_dim.entry(neighbor).or_insert(false);
                }
            }
        }
    }

    for _ in 0..6 {
        let mut next_cycle: HashMap<[i32; 3], bool> = HashMap::new();
        for (coord, active) in &poc_dim {
            let neighbors = get_3d_neighbors(coord);
            let actives = neighbors
                .iter()
                .filter(|&c| *poc_dim.get(c).unwrap_or(&false))
                .count();
            let next_active = match active {
                true => actives == 2 || actives == 3,
                false => actives == 3,
            };
            next_cycle.insert(*coord, next_active);
            if !active && next_active {
                for neighbor in neighbors {
                    next_cycle.entry(neighbor).or_insert(false);
                }
            }
        }
        poc_dim = next_cycle;
    }
    poc_dim
        .iter()
        .fold(0, |acc, (_, &s)| if s { acc + 1 } else { acc })
}

fn part2() -> i32 {
    let mut poc_dim: HashMap<[i32; 4], bool> = HashMap::new();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, char) in line.chars().enumerate() {
            let active = match char {
                '.' => false,
                '#' => true,
                _ => panic!(),
            };
            let coord = [i.try_into().unwrap(), j.try_into().unwrap(), 0, 0];
            poc_dim.insert(coord, active);
            if active {
                for neighbor in get_4d_neighbors(&coord) {
                    poc_dim.entry(neighbor).or_insert(false);
                }
            }
        }
    }

    for _ in 0..6 {
        let mut next_cycle: HashMap<[i32; 4], bool> = HashMap::new();
        for (coord, active) in &poc_dim {
            let neighbors = get_4d_neighbors(coord);
            let actives = neighbors
                .iter()
                .filter(|&c| *poc_dim.get(c).unwrap_or(&false))
                .count();
            let next_active = match active {
                true => actives == 2 || actives == 3,
                false => actives == 3,
            };
            next_cycle.insert(*coord, next_active);
            if !active && next_active {
                for neighbor in neighbors {
                    next_cycle.entry(neighbor).or_insert(false);
                }
            }
        }
        poc_dim = next_cycle;
    }
    poc_dim
        .iter()
        .fold(0, |acc, (_, &s)| if s { acc + 1 } else { acc })
}

fn get_3d_neighbors(coord: &[i32; 3]) -> HashSet<[i32; 3]> {
    let mut coords = HashSet::new();
    coords.insert(*coord);
    coords = coords
        .iter()
        .flat_map(|[x, y, z]| [[x - 1, *y, *z], [*x, *y, *z], [x + 1, *y, *z]])
        .flat_map(|[x, y, z]| [[x, y - 1, z], [x, y, z], [x, y + 1, z]])
        .flat_map(|[x, y, z]| [[x, y, z - 1], [x, y, z], [x, y, z + 1]])
        .collect();
    coords.remove(coord);
    assert_eq!(coords.len(), 26);
    coords
}

fn get_4d_neighbors(coord: &[i32; 4]) -> HashSet<[i32; 4]> {
    let mut coords = HashSet::new();
    coords.insert(*coord);
    coords = coords
        .iter()
        .flat_map(|[x, y, z, w]| [[x - 1, *y, *z, *w], [*x, *y, *z, *w], [x + 1, *y, *z, *w]])
        .flat_map(|[x, y, z, w]| [[x, y - 1, z, w], [x, y, z, w], [x, y + 1, z, w]])
        .flat_map(|[x, y, z, w]| [[x, y, z - 1, w], [x, y, z, w], [x, y, z + 1, w]])
        .flat_map(|[x, y, z, w]| [[x, y, z, w - 1], [x, y, z, w], [x, y, z, w + 1]])
        .collect();
    coords.remove(coord);
    assert_eq!(coords.len(), 80);
    coords
}
