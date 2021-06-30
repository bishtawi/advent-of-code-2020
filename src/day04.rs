use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = "input/day04.txt";

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

impl Passport {
    fn valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }

    fn valider(&self) -> bool {
        lazy_static! {
            static ref HGT: Regex = Regex::new(r"^(?P<height>\d+)(?P<units>cm|in)$").unwrap();
            static ref HCL: Regex = Regex::new(r"^\#[0-9a-f]{6}$").unwrap();
            static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
            static ref FOUR_DIGIT: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        if !self.valid() {
            return false;
        }

        // These checks should probably be their own methods...

        // byr
        if !FOUR_DIGIT.is_match(self.byr.as_ref().unwrap()) {
            return false;
        }
        match self.byr.as_ref().unwrap().parse::<i32>() {
            Ok(val) => {
                if val < 1920 || val > 2002 {
                    return false;
                }
            }
            _ => return false,
        }

        // iyr
        if !FOUR_DIGIT.is_match(self.iyr.as_ref().unwrap()) {
            return false;
        }
        match self.iyr.as_ref().unwrap().parse::<i32>() {
            Ok(val) => {
                if val < 2010 || val > 2020 {
                    return false;
                }
            }
            _ => return false,
        }

        // eyr
        if !FOUR_DIGIT.is_match(self.eyr.as_ref().unwrap()) {
            return false;
        }
        match self.eyr.as_ref().unwrap().parse::<i32>() {
            Ok(val) => {
                if val < 2020 || val > 2030 {
                    return false;
                }
            }
            _ => return false,
        }

        // hgt
        match HGT.captures(self.hgt.as_ref().unwrap()) {
            Some(r) => {
                let height = match r["height"].parse::<i32>() {
                    Ok(val) => val,
                    _ => return false,
                };
                match &r["units"] {
                    "cm" => {
                        if height < 150 || height > 193 {
                            return false;
                        }
                    }
                    "in" => {
                        if height < 59 || height > 76 {
                            return false;
                        }
                    }
                    _ => return false,
                };
            }
            _ => return false,
        }

        // hcl
        if !HCL.is_match(self.hcl.as_ref().unwrap()) {
            return false;
        }

        // ecl
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .contains(&self.ecl.as_ref().unwrap().as_str())
        {
            return false;
        }

        // pid
        if !PID.is_match(self.pid.as_ref().unwrap()) {
            return false;
        }

        return true;
    }
}

pub fn solve() {
    println!("{}: {} | {}", INPUT, part1(), part2());
}

fn part1() -> i32 {
    let passports = parse();

    let mut valid = 0;
    for passport in passports {
        if passport.valid() {
            valid += 1;
        }
    }

    return valid;
}

fn part2() -> i32 {
    let passports = parse();

    let mut valid = 0;
    for passport in passports {
        if passport.valider() {
            valid += 1;
        }
    }

    return valid;
}

fn parse() -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut last = Passport::default();

    let file = File::open(INPUT).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();

        if line == "" {
            passports.push(last);
            last = Passport::default();
            continue;
        }

        let parts: Vec<&str> = line.split(" ").collect();
        for part in parts {
            let pair: Vec<&str> = part.split(":").collect();
            assert!(pair.len() == 2);
            let val = Some(pair[1].to_string());
            match pair[0] {
                "byr" => last.byr = val,
                "iyr" => last.iyr = val,
                "eyr" => last.eyr = val,
                "hgt" => last.hgt = val,
                "hcl" => last.hcl = val,
                "ecl" => last.ecl = val,
                "pid" => last.pid = val,
                "cid" => last.cid = val,
                _ => assert!(false),
            }
        }
    }
    passports.push(last);

    return passports;
}
