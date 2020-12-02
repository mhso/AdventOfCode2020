extern crate util;

pub fn part_one() -> () {
    let lines = util::read_input(2);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut valid_passwords = 0;
    for i in 0..list.len()-1 {
        let split_space: Vec<&str> = list[i].split(" ").collect();
        let range = split_space[0];
        let split_range: Vec<&str> = range.split("-").collect();
        let min : u32 = split_range[0].trim().parse().unwrap();
        let max : u32 = split_range[1].trim().parse().unwrap();
        let letter = split_space[1].chars().nth(0).unwrap();
        let password = split_space[2].trim();

        let mut letter_freq = 0;

        for c in password.chars() {
            if c == letter {
                letter_freq += 1;
            }
        }

        if letter_freq >= min && letter_freq <= max {
            valid_passwords += 1;
        }
    }
    println!("Part 1: {}", valid_passwords);
}

pub fn part_two() -> () {
    let lines = util::read_input(2);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut valid_passwords = 0;
    for i in 0..list.len()-1 {
        let split_space: Vec<&str> = list[i].split(" ").collect();
        let range = split_space[0];
        let split_range: Vec<&str> = range.split("-").collect();
        let index_1 : usize = split_range[0].trim().parse().unwrap();
        let index_2 : usize = split_range[1].trim().parse().unwrap();
        let letter = split_space[1].chars().nth(0).unwrap();
        let password = split_space[2].trim();

        let mut times_seen = 0;
        for (i, c) in password.char_indices() {
            if c == letter {
                if i == (index_1 - 1) || i == (index_2 - 1) {
                    times_seen += 1;
                }
            }
        }
        if times_seen == 1 {
            valid_passwords += 1;
        }
    }
    println!("Part 2: {}", valid_passwords);
}