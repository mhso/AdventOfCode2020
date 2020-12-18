extern crate util;

pub fn part_one() -> () {
	let lines = util::read_input(13);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let time_depature : u32 = list[0].trim().parse().unwrap();
    let depatures : Vec<&str> = list[1].split(",").collect();

    let mut earliest_depature = 0;
    let mut least_wait = u32::MAX;

    for depature in depatures {
        if depature != "x" {
            let num : u32 = depature.trim().parse().unwrap();
            let time_after = num - (time_depature % num);
            if time_after < least_wait {
                least_wait = time_after;
                earliest_depature = num;
            }
        }
    }

    println!("Bus: {}", earliest_depature);
    println!("Part 1: {}", earliest_depature * least_wait);
}

pub fn part_two() -> () {
    // Solved the following equations via. WolframAlpha, cause lazy.
    // x % 29 = 0
    // x + 23 % 37 = 0
    // x + 29 % 433 = 0
    // x + 42 % 13 = 0
    // x + 43 % 17 = 0
    // x + 48 % 19 = 0
    // x + 52 % 23 = 0
    // x + 60 % 977 = 0
    // x + 101 % 41 = 0
    // Answer: x = 534035653563227
}