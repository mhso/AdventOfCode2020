extern crate util;
use std::collections::HashMap;

fn apply_mask(num: u64, mask: &str) -> u64 {
    let mut fmt = format!("{:b}", num);
    for i in 0..36-fmt.len() {
        fmt = "0".to_string() + &fmt;
    }
    let mut input_chars: Vec<char> = fmt.chars().collect();
    let mask_chars: Vec<char> = mask.chars().collect();
    for i in 0..input_chars.len() {
        if mask_chars[i] != 'X' {
            input_chars[i] = mask_chars[i];
        }
    }
    let to_binary_str : String = input_chars.into_iter().collect();
    return isize::from_str_radix(&to_binary_str, 2).unwrap() as u64;
}

pub fn part_one() -> () {
	let lines = util::read_input(14);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut curr_mask = "";

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        if line.starts_with("mask") {
            let split: Vec<&str> = line.split(" = ").collect();
            curr_mask = split[1];
        }
        else {
            let split_1: Vec<&str> = line.split("[").collect();
            let split_2: Vec<&str> = split_1[1].split("] = ").collect();
            let mem_address : u64 = split_2[0].parse().unwrap();
            let value : u64 = split_2[1].parse().unwrap();
            let masked = apply_mask(value, curr_mask);
            memory.insert(mem_address, masked);
        }
    }

    let mut sum = 0;
    for (k, v) in memory {
        sum += v;
    }
    println!("Part 1: {}", sum);
}

fn permutations(amount : usize) -> Vec<Vec<char>> {
    let mut bitstring: Vec<Vec<char>> = Vec::new();
    for i in 0..u32::pow(2, amount as u32) {
        let mut bits = format!("{:b}", i);
        for _ in 0..amount-bits.len() {
            bits = "0".to_string() + &bits;
        }
        let chars : Vec<char> = bits.chars().collect();
        bitstring.push(chars);
    }
    return bitstring;
}

fn apply_mask_v2(num: u64, mask: &str) -> Vec<u64> {
    let mut fmt = format!("{:b}", num);
    for i in 0..36-fmt.len() {
        fmt = "0".to_string() + &fmt;
    }
    let mut input_chars: Vec<char> = fmt.chars().collect();

    let mask_chars: Vec<char> = mask.chars().collect();
    let mut x_indices = Vec::new();
    for i in 0..mask_chars.len() {
        if mask_chars[i] == 'X' {
            x_indices.push(i);
        }
        else if mask_chars[i] == '1' {
            input_chars[i] = mask_chars[i];
        }
    }

    let permuts = permutations(x_indices.len());
    let mut numbers = Vec::new();

    for chars in permuts {
        let mut final_char_list = input_chars.clone();
        for i in 0..x_indices.len() as usize {
            let index = x_indices[i];
            final_char_list[index] = chars[i];
        }
        let to_binary_str : String = final_char_list.into_iter().collect();
        let to_num = isize::from_str_radix(&to_binary_str, 2).unwrap() as u64;
        numbers.push(to_num);
    }

    return numbers;
}

pub fn part_two() -> () {
	let lines = util::read_input(14);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut curr_mask = "";

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        if line.starts_with("mask") {
            let split: Vec<&str> = line.split(" = ").collect();
            curr_mask = split[1];
        }
        else {
            let split_1: Vec<&str> = line.split("[").collect();
            let split_2: Vec<&str> = split_1[1].split("] = ").collect();
            let mem_address : u64 = split_2[0].parse().unwrap();
            let value : u64 = split_2[1].parse().unwrap();
            let masked_adress = apply_mask_v2(mem_address, curr_mask);
            for mem_adress in masked_adress {
                memory.insert(mem_adress, value);
            }
        }
    }

    let mut sum = 0;
    for (k, v) in memory {
        sum += v;
    }
    println!("Part 2: {}", sum);
}