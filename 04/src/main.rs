use std::collections::HashMap;
use regex::Regex;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE_NAME).expect("Opening file failed")
                                                  .parse::<String>().expect("Converting to string failed");

    let candidates: Vec<Passport> = input.split("\n\n").map( |x| Passport::from(x) ).collect();

    println!("Basic Valid Passports: {}", candidates.iter().filter(|x| x.basic_valid() ).count());

    println!("Enhanced Valid Passports: {}", candidates.iter().filter(|x| x.enhanced_valid() ).count());
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn from(info: &str) -> Passport {
        let mut passport_info = HashMap::new();
        for kv in info.split_whitespace() {
            let kv = kv.split(":").collect::<Vec<&str>>();
            passport_info.insert(kv[0], kv[1]);
        }
        Passport {
            byr: passport_info.get("byr").and_then(|x| Some(String::from(*x))),
            iyr: passport_info.get("iyr").and_then(|x| Some(String::from(*x))),
            eyr: passport_info.get("eyr").and_then(|x| Some(String::from(*x))),
            hgt: passport_info.get("hgt").and_then(|x| Some(String::from(*x))),
            hcl: passport_info.get("hcl").and_then(|x| Some(String::from(*x))),
            ecl: passport_info.get("ecl").and_then(|x| Some(String::from(*x))),
            pid: passport_info.get("pid").and_then(|x| Some(String::from(*x))),
        }
    }

    fn basic_valid(&self) -> bool {
        self.byr.is_some()
        && self.iyr.is_some()
        && self.eyr.is_some()
        && self.hgt.is_some()
        && self.hcl.is_some()
        && self.ecl.is_some()
        && self.pid.is_some()
    }

    fn enhanced_valid(&self) -> bool {
        let re_hcl = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        let re_pid = Regex::new(r"^\d{9}$").unwrap();

        self.basic_valid()
        && ((1920 <= self.byr.as_ref().unwrap().parse::<usize>().unwrap()) && ((&self.byr).as_ref().unwrap().parse::<usize>().unwrap() <= 2002))
        && ((2010 <= self.iyr.as_ref().unwrap().parse::<usize>().unwrap()) && ((&self.iyr).as_ref().unwrap().parse::<usize>().unwrap() <= 2020))
        && ((2020 <= self.eyr.as_ref().unwrap().parse::<usize>().unwrap()) && ((&self.eyr).as_ref().unwrap().parse::<usize>().unwrap() <= 2030))
        && self.hgt_valid()
        && re_hcl.is_match(self.hcl.as_ref().unwrap())
        && re_ecl.is_match(self.ecl.as_ref().unwrap())
        && re_pid.is_match(self.pid.as_ref().unwrap())
    }

    fn hgt_valid(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(height) => {
                match height.chars().rev().take(2).collect::<String>().chars().rev().collect::<String>().as_str() {
                    "in" => {
                        let value: usize = height.chars().take(height.len() - 2).collect::<String>().parse().unwrap();
                        (59 <= value) && (value <= 76)
                    },
                    "cm" => {
                        let value: usize = height.chars().take(height.len() - 2).collect::<String>().parse().unwrap();
                        (150 <= value) && (value <= 193)
                    },
                    _ => {
                        false
                    }
                }
            }
        }
    }
}
