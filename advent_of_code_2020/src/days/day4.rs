extern crate util;
extern crate regex;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

pub fn part_one() -> () {
    let lines = util::read_input(4);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut valid_passports = 0;
    let mut curr_fields: HashSet<&str> = HashSet::new();

    for i in 0..list.len() {
        let line = list[i].trim();
        if line == "" {
            if curr_fields.len() == 7 {
                valid_passports += 1;
            }
            curr_fields.drain();
        }
        else {
            let split = line.split(" ");
            for kv_pair in split {
                let kv_split: Vec<&str> = kv_pair.split(":").collect();
                if kv_split[0] != "cid" {
                    curr_fields.insert(kv_split[0]);
                }
            }
        }
    }

    println!("Part 1: {}", valid_passports);
}

fn get_regex_patterns() -> HashMap<&'static str, Regex> {
    let mut map: HashMap<&str, Regex> = HashMap::new();
    map.insert("byr", Regex::new(r"^(19[2-8][0-9]|199[0-9]|200[0-2])$").unwrap());
    map.insert("iyr", Regex::new(r"^(201[0-9]|2020)$").unwrap());
    map.insert("eyr", Regex::new(r"^(202[0-9]|2030)$").unwrap());
    map.insert("hgt", Regex::new(r"^(1[5-8][0-9]|19[0-3])cm$|^(59|6[0-9]|7[0-6])in$").unwrap());
    map.insert("hcl", Regex::new(r"^(#[0-9a-f]{6})$").unwrap());
    map.insert("ecl", Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
    map.insert("pid", Regex::new(r"^[0-9]{9}$").unwrap());
    return map;
}

pub fn part_two() -> () {
    let lines = util::read_input(4);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut valid_passports = 0;
    let mut curr_fields: HashSet<&str> = HashSet::new();
    let regex_map: HashMap<&str, Regex> = get_regex_patterns();

    for i in 0..list.len() {
        let line = list[i].trim();
        if line == "" {
            if curr_fields.len() == 7 {
                valid_passports += 1;
            }
            curr_fields.drain();
        }
        else {
            let split = line.split(" ");
            for kv_pair in split {
                let kv_split: Vec<&str> = kv_pair.split(":").collect();
                if kv_split[0] != "cid" {
                    if regex_map[kv_split[0]].is_match(kv_split[1]) {
                        curr_fields.insert(kv_split[0]);
                    }
                }
            }
        }
    }

    println!("Part 2: {}", valid_passports);
}