extern crate util;

pub fn part_one() -> () {
    let lines = util::read_input(1);
    let list: Vec<&str> = lines.split("\n").collect();
    for i in 0..list.len()-1 {
        let num_1: i32 = list[i].trim().parse().unwrap();
        for j in 0..list.len()-1 {
            if i != j {
                let num_2:i32 = list[j].trim().parse().unwrap();
                if num_1 + num_2 == 2020 {
                    println!("{}", num_1 * num_2);
                }
            }
        }
    }
}

pub fn part_two() -> () {
    let lines = util::read_input(1);
    let list: Vec<&str> = lines.split("\n").collect();
    for i in 0..list.len()-1 {
        let num_1: i32 = list[i].trim().parse().unwrap();
        for j in 0..list.len()-1 {
            let num_2:i32 = list[j].trim().parse().unwrap();
            for k in 0..list.len()-1 {
                let num_3:i32 = list[k].trim().parse().unwrap();
                if num_1 + num_2 + num_3 == 2020 {
                    println!("{}", num_1 * num_2 * num_3);
                }
            }
        }
    }
}