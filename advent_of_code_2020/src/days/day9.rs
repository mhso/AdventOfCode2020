extern crate util;
extern crate queues;
use queues::*;

pub fn part_one() -> () {
	let lines = util::read_input(9);
    let list: Vec<&str> = lines.split("\n").collect();
    
    for i in 25..list.len()-1 {
        let number : i64 = list[i].trim().parse().unwrap();
        let mut valid = false;
        for j in i-25..i {
            let num_1 : i64 = list[j].trim().parse().unwrap();
            for k in i-25..i {
                if j != k {
                    let num_2 : i64 = list[k].trim().parse().unwrap();
                    if num_1 + num_2 == number {
                        valid = true;
                        break;
                    }
                }
            }
            if valid {
                break;
            }
        }
        if !valid {
            println!("Part 1: {}", number);
            break;
        }
    }
}

fn print_numbers(queue: &mut Queue<i64>) {
    let mut min : i64 = 100000000000;
    let mut max : i64 = 0;
    while queue.size() > 0 {
        let number = queue.remove().unwrap();
        if number < min {
            min = number;
        }
        else if number > max {
            max = number;
        }
    }
    println!("Part 2: {}", (min + max));
}

pub fn part_two() -> () {
	let lines = util::read_input(9);
    let list: Vec<&str> = lines.split("\n").collect();
    let invalid_num : i64 = 1398413738;
    
    for i in 2..list.len()-1 {
        let mut acc : Queue<i64> = Queue::new();
        let mut sum : i64 = 0;

        for j in 0..list.len()-1 {
            let number : i64 = list[j].trim().parse().unwrap();
            sum += number;
            acc.add(number).unwrap();

            if acc.size() == i {
                if sum == invalid_num {
                    print_numbers(&mut acc);
                    return;
                }
                sum -= acc.remove().unwrap();
            }
        }
    }
}