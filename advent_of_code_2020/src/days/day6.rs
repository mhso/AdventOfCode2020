extern crate util;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn part_one() -> () {
	let lines = util::read_input(6);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut group_chars: HashSet<char> = HashSet::new();
    let mut sum_answers = 0;

    for i in 0..list.len() {
        let line = list[i].trim();
        if line == "" {
            sum_answers += group_chars.len();
            group_chars.drain();
        }
        for c in line.chars() {
            group_chars.insert(c);
        }
    }

    println!("Part 1: {}", sum_answers);
}

pub fn part_two() -> () {
	let lines = util::read_input(6);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut answers: HashMap<char, u32> = HashMap::new();
    let mut group_size = 0;
    let mut sum_answers = 0;

    for i in 0..list.len() {
        let line = list[i].trim();
        if line == "" {
            let mut amount_answered = 0;
            for (_, v) in &answers {
                if *v == group_size {
                    amount_answered += 1;
                }
            }
            sum_answers += amount_answered;
            answers.drain();
            group_size = 0;
        }
        else {
            group_size += 1;
        }
        for c in line.chars() {
            match answers.get(&c) {
                Some(v) => answers.insert(c, v + 1),
                None => answers.insert(c, 1)
            };
        }
    }

    println!("Part 1: {}", sum_answers);
}