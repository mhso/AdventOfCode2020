extern crate util;

pub fn part_one() -> () {
	let lines = util::read_input(10);
    let list : Vec<&str> = lines.split("\n").collect();
    let mut numbers : Vec<u32> = list[..list.len()-1].iter().map(|&x| x.trim().parse::<u32>().unwrap()).collect();
    numbers.sort();
    numbers.push(numbers[numbers.len()-1] + 3);

    let mut one_diff_counts = 0;
    let mut three_diff_counts = 0;

    let mut prev = 0;

    for i in 0..numbers.len() {
        let current = numbers[i];
        let diff = current - prev;
        if diff == 1 {
            one_diff_counts += 1;
        }
        else if diff == 3 {
            three_diff_counts += 1;
        }
        prev = current;
    }

    println!("Part 1: {}", one_diff_counts * three_diff_counts);
}

pub fn part_two() -> () {
	let lines = util::read_input(10);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut numbers : Vec<i64> = list[..list.len()-1].iter().map(|&x| x.trim().parse::<i64>().unwrap()).collect();
    numbers.sort();

    let mut possibilities : Vec<i64> = vec![0; numbers.len()];
    possibilities[0] = 1;
    possibilities[1] = 2;
    possibilities[2] = 4;

    for i in 3..numbers.len() {
        let number = numbers[i];
        let mut sum_possibilities : i64 = 0;
        for j in i-3..i {
            if number - numbers[j] < 4 {
                sum_possibilities += possibilities[j];
            }
        }
        possibilities[i] = sum_possibilities;
    }
    println!("Part 2: {}", possibilities[possibilities.len()-1]);
}