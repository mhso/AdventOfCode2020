extern crate util;
use std::collections::HashMap;

fn play_game(rounds: usize, part: u32) {
    let lines = util::read_input(15);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let mut numbers : HashMap<u32, u32> = HashMap::new();

    let starting_nums : Vec<u32> = list[0].split(",").map(|x| x.trim().parse::<u32>().unwrap()).collect();
    for i in 0..starting_nums.len() {
        numbers.insert(starting_nums[i], (i+1) as u32);
    }

    let mut num = 0;
    for turn in starting_nums.len()+1..rounds {
        let new_num = match numbers.get(&num) {
            Some(v) => (turn as u32) - v,
            None => 0
        };
        numbers.insert(num, turn as u32);
        num = new_num;
    }
    println!("Part {}: {}", part, num);
}

pub fn part_one() -> () {
	play_game(2020, 1);
}

pub fn part_two() -> () {
	play_game(30000000, 1);
}