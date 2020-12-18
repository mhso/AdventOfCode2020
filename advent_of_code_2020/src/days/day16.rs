extern crate util;
use std::collections::HashSet;

fn get_rules(lines: &Vec<&str>) -> Vec<(u32, u32, u32, u32)> {
    let mut rules : Vec<(u32, u32, u32, u32)> = Vec::new();
    for i in 0..lines.len()-1 {
        let line = lines[i].trim();
        if line.contains(":") {
            match line {
                "your ticket:" => (),
                "nearby tickets:" => (),
                _ => {
                    let split: Vec<&str> = line.split(": ").collect();
                    let split_2: Vec<&str> = split[1].split(" or ").collect();
                    let split_or_1: Vec<&str> = split_2[0].split("-").collect();
                    let split_or_2: Vec<&str> = split_2[1].split("-").collect();
                    let min_1 = split_or_1[0].parse().unwrap();
                    let max_1 = split_or_1[1].parse().unwrap();
                    let min_2 = split_or_2[0].parse().unwrap();
                    let max_2 = split_or_2[1].parse().unwrap();
                    rules.push((min_1, max_1, min_2, max_2));
                }
            }
        }
    }
    return rules;
}

fn rule_valid(rule: &(u32, u32, u32, u32), value : u32) -> bool {
    return (value >= rule.0 && value <= rule.1) || (value >= rule.2 && value <= rule.3);
}

pub fn part_one() -> () {
	let lines = util::read_input(16);
    let list: Vec<&str> = lines.split("\n").collect();

    let rules = get_rules(&list);
    
    let mut nearby_tickets = false;
    let mut invalid_sum = 0;

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        if nearby_tickets {
            let split: Vec<&str> = line.split(",").collect();
            for num_str in split {
                let num : u32 = num_str.parse().unwrap();
                let mut valid = false;
                for rule_tup in rules.iter() {
                    if rule_valid(rule_tup, num) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    invalid_sum += num;
                }
            }
        }
        else if line == "nearby tickets:" {
            nearby_tickets = true;
        }
    }
    println!("Part 1: {}", invalid_sum);
}

pub fn part_two() -> () {
	let lines = util::read_input(16);
    let list: Vec<&str> = lines.split("\n").collect();

    let rules = get_rules(&list);

    let mut my_ticket_seen = false;
    let mut nearby_ticket_seen = false;
    let mut valid_tickets = Vec::new();
    let mut my_ticket : Vec<u32> = Vec::new();

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        if line == "your ticket:" {
            my_ticket_seen = true;
        }
        else if line == "nearby tickets:" {
            nearby_ticket_seen = true;
        }
        else if my_ticket_seen {
            let split: Vec<&str> = line.split(",").collect();
            let numbers : Vec<u32> = split.iter().map(|&x| x.trim().parse::<u32>().unwrap()).collect();
            my_ticket = numbers;
            my_ticket_seen = false;
        }
        else if nearby_ticket_seen {
            let split: Vec<&str> = line.split(",").collect();
            let mut all_valid = true;
            let numbers : Vec<u32> = split.iter().map(|&x| x.trim().parse::<u32>().unwrap()).collect();
            for num in numbers.iter() {
                let mut valid = false;
                for rule_tup in rules.iter() {
                    if rule_valid(rule_tup, *num) {
                        valid = true;
                        break;
                    }
                }
                if !valid {
                    all_valid = false;
                    break;
                }
            }
            if all_valid {
                valid_tickets.push(numbers);
            }
        }
    }

    let mut valid_orderings = vec![vec![]; rules.len()];

    for rule in 0..rules.len() {
        for rule_order in 0..rules.len() {
            let mut valid = true;
            for ticket in 0..valid_tickets.len() {
                if !rule_valid(&rules[rule], valid_tickets[ticket][rule_order]) {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_orderings[rule_order].push(rule as u32);
            }
        }
    }

    let mut final_ordering : HashSet<&u32> = HashSet::new();
    let mut rule_indices = Vec::new();

    while final_ordering.len() < rules.len() {
        for rule_order in 0..valid_orderings.len() {
            let mut pos_orderings = Vec::new();
            for rule in valid_orderings[rule_order].iter() {
                if !final_ordering.contains(&rule) {
                    pos_orderings.push(rule);
                }
            }
            if pos_orderings.len() == 1 {
                final_ordering.insert(pos_orderings[0]);
                if pos_orderings[0] < &6 {
                    rule_indices.push(rule_order);
                }
            }
        }
    }

    let mut final_answer : i64 = 1;
    for rule in rule_indices {
        let ticket_value = my_ticket[rule] as i64;
        final_answer *= ticket_value;
    }
    println!("Part 2: {}", final_answer);
}