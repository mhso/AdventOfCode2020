extern crate util;
use std::collections::HashSet;

pub fn part_one() -> () {
	let lines = util::read_input(8);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let mut seen_lines : HashSet<i32> = HashSet::new();
    let mut pc : i32 = 0;
    let mut acc = 0;
    loop {
        if seen_lines.contains(&pc) {
            break;
        }
        seen_lines.insert(pc);
        let split : Vec<&str> = list[pc as usize].trim().split(" ").collect();
        let instr = split[0];
        let value : i32 = split[1].parse().unwrap();
        match instr {
            "acc" => acc += value,
            "jmp" => pc += value - 1,
            _ => ()
        }
        pc += 1
    }
    println!("Part 1: {}", acc);
}

pub fn part_two() -> () {
	let lines = util::read_input(8);
	let list: Vec<&str> = lines.split("\n").collect();

    for i in 0..list.len()-1 {
        let mut pc : i32 = 0;
        let mut seen_lines : HashSet<i32> = HashSet::new();
        let mut acc = 0;
        let mut valid = true;
        loop {
            if seen_lines.contains(&pc) || pc as usize >= list.len() || pc < 0 {
                valid = false;
                break;
            }
            if pc as usize == list.len()-1 {
                valid = true;
                break;
            }
            seen_lines.insert(pc);
            let split : Vec<&str> = list[pc as usize].trim().split(" ").collect();
            let instr = split[0];
            let value : i32 = split[1].parse().unwrap();
            match instr {
                "acc" => acc += value,
                "jmp" => {
                    if pc as usize == i {
                        (); // Treat as nop.
                    }
                    else {
                        pc += value - 1
                    }
                },
                _ => {
                    if pc as usize == i {
                        pc += value - 1; // Treat as jmp.
                    }
                }
            }
            pc += 1
        }
        if valid {
            println!("Part 2: {}", acc);
            break;
        }
    }
}