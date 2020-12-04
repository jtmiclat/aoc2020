use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

use regex::Regex;
pub fn solve_a(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut valid_hashset: HashSet<String> = HashSet::new();
    valid_hashset.insert("byr".to_string());
    valid_hashset.insert("iyr".to_string());
    valid_hashset.insert("eyr".to_string());
    valid_hashset.insert("hgt".to_string());
    valid_hashset.insert("hcl".to_string());
    valid_hashset.insert("ecl".to_string());
    valid_hashset.insert("pid".to_string());

    let mut valid_hashset_2: HashSet<String> = HashSet::new();
    valid_hashset_2.insert("byr".to_string());
    valid_hashset_2.insert("iyr".to_string());
    valid_hashset_2.insert("eyr".to_string());
    valid_hashset_2.insert("hgt".to_string());
    valid_hashset_2.insert("hcl".to_string());
    valid_hashset_2.insert("ecl".to_string());
    valid_hashset_2.insert("pid".to_string());
    valid_hashset_2.insert("cid".to_string());

    let passports: Vec<&str> = file.split("\n\n").collect();
    let valid_passwords: i32 = passports.len() as i32;
    let mut invalid_passwords: i32 = 0;
    for passport in passports.iter() {
        let fields: Vec<String> = passport
            .split_whitespace()
            .map(|f| f.split(":").next().unwrap().to_string())
            .collect();
        let set: HashSet<String> = HashSet::from_iter(fields);
        if !(set == valid_hashset || set == valid_hashset_2) {
            invalid_passwords = invalid_passwords + 1;
        }
    }
    println!(
        "Total Valid passports: {}",
        valid_passwords - invalid_passwords
    )
}

pub fn solve_b(filename: &str) {
    let file: String = fs::read_to_string(filename).expect("Error opening file");
    let mut valid_hashset: HashSet<String> = HashSet::new();
    valid_hashset.insert("byr".to_string());
    valid_hashset.insert("iyr".to_string());
    valid_hashset.insert("eyr".to_string());
    valid_hashset.insert("hgt".to_string());
    valid_hashset.insert("hcl".to_string());
    valid_hashset.insert("ecl".to_string());
    valid_hashset.insert("pid".to_string());

    let mut valid_hashset_2: HashSet<String> = HashSet::new();
    valid_hashset_2.insert("byr".to_string());
    valid_hashset_2.insert("iyr".to_string());
    valid_hashset_2.insert("eyr".to_string());
    valid_hashset_2.insert("hgt".to_string());
    valid_hashset_2.insert("hcl".to_string());
    valid_hashset_2.insert("ecl".to_string());
    valid_hashset_2.insert("pid".to_string());
    valid_hashset_2.insert("cid".to_string());
    let passports: Vec<&str> = file.split("\n\n").collect();
    let valid_passwords: i32 = passports.len() as i32;
    let mut invalid_passwords: i32 = 0;
    for passport in passports.iter() {
        let fields: Vec<String> = passport
            .split_whitespace()
            .map(|f| f.split(":").next().unwrap().to_string())
            .collect();
        let set: HashSet<String> = HashSet::from_iter(fields);
        if !(set == valid_hashset || set == valid_hashset_2) {
            invalid_passwords = invalid_passwords + 1;
        } else {
            for kv in passport.split_whitespace() {
                let mut kv_ = kv.split(":");
                let k = kv_.next().unwrap();
                let v = kv_.next().unwrap();
                if k == "byr" {
                    let num = v.parse::<i32>();
                    match num {
                        Ok(val) => {
                            if val < 1920 || val > 2002 {
                                println!("wrong byr {}", val);
                                invalid_passwords = invalid_passwords + 1;
                                break;
                            }
                        }
                        Err(_) => (),
                    }
                } else if k == "iyr" {
                    let num = v.parse::<i32>();
                    match num {
                        Ok(val) => {
                            if val < 2010 || val > 2020 {
                                println!("wrong iyr {}", val);
                                invalid_passwords = invalid_passwords + 1;
                                break;
                            }
                        }
                        Err(_) => (),
                    }
                } else if k == "eyr" {
                    let num = v.parse::<i32>();
                    match num {
                        Ok(val) => {
                            if val < 2020 || val > 2030 {
                                println!("wrong eyr {}", val);
                                invalid_passwords = invalid_passwords + 1;
                                break;
                            }
                        }
                        Err(_) => (),
                    }
                } else if k == "hgt" {
                    if v.to_string().contains("cm") {
                        let num: i32 = v.to_string().replace("cm", "").parse().unwrap();
                        if num < 150 || num > 193 {
                            invalid_passwords = invalid_passwords + 1;
                            println!("wrong hgt {}", v);
                            break;
                        }
                    } else if v.to_string().contains("in") {
                        let num: i32 = v.to_string().replace("in", "").parse().unwrap();
                        if num < 59 || num > 76 {
                            invalid_passwords = invalid_passwords + 1;
                            println!("wrong hgt {}", v);
                            break;
                        }
                    } else {
                        invalid_passwords = invalid_passwords + 1;
                        println!("wrong hgt {}", v);
                        break;
                    }
                } else if k == "hcl" {
                    let re = Regex::new(r"^#[0-9A-F]{6}").unwrap();
                    if !re.is_match(&v.to_string().to_uppercase()) {
                        println!("wrong hcl {}", v);
                        invalid_passwords = invalid_passwords + 1;

                        break;
                    }
                } else if k == "ecl" {
                    let n = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if !n.iter().any(|&i| i == v) {
                        println!("wrong ecl {}", v);
                        invalid_passwords = invalid_passwords + 1;
                        break;
                    }
                } else if k == "pid" {
                    if !(v.len() == 9 && v.parse::<i32>().is_ok()) {
                        println!("wrong pid {}", v);
                        invalid_passwords = invalid_passwords + 1;
                        break;
                    }
                }
            }
        }
    }
    println!(
        "Total Valid passports: {}",
        valid_passwords - invalid_passwords
    )
}
