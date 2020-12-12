extern crate util;

fn get_occupied_neighbors(config: &Vec<Vec<char>>, i : i32, j : i32) -> u32 {
    let mut count = 0;
    let max_y = config.len() as i32;
    let max_x = config[0].len() as i32;
    for y in i-1..i+2 {
        if y >= 0 && y < max_y {
            for x in j-1..j+2 {
                if x >= 0 && x < max_x && (x != j || y != i) {
                    if config[y as usize][x as usize] == '#' {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

fn print_vec(vector: &Vec<Vec<char>>) {
    println!("[");
    for i in 0..vector.len() {
        print!(" ");
        for j in 0..vector[i].len() {
            print!("{}", vector[i][j]);
        }
        println!();
    }
    println!("]")
}

pub fn part_one() -> () {
	let lines = util::read_input(11);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut curr_config : Vec<Vec<char>> = list[..list.len()-1].iter().map(|&x| x.trim().chars().collect()).collect();

    loop {
        let mut next_config = curr_config.clone();
        let mut changed = false;
        for i in 0..curr_config.len() {
            for j in 0..curr_config[i].len() {
                let c = curr_config[i][j];
                if c == 'L' {
                    if get_occupied_neighbors(&curr_config, i as i32, j as i32) == 0 {
                        next_config[i][j] = '#';
                        changed = true;
                    }
                }
                else if c == '#' {
                    if get_occupied_neighbors(&curr_config, i as i32, j as i32) >= 4 {
                        next_config[i][j] = 'L';
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            let mut occupied_seats = 0;
            for row in curr_config {
                for c in row {
                    if c == '#' {
                        occupied_seats += 1;
                    }
                }
            }
            println!("Part 1: {}", occupied_seats);
            break;
        }
        curr_config = next_config;
    }
}

fn get_occupied_neighbors_v2(config: &Vec<Vec<char>>, i : i32, j : i32) -> u32 {
    let mut count = 0;
    let max_y = config.len() as i32;
    let max_x = config[0].len() as i32;
    let directions = [
        (1, 1), (0, 1), (-1, 1), (-1, 0),
        (-1, -1), (0, -1), (1, -1), (1, 0)
    ];

    for (y_dir, x_dir) in &directions {
        let mut y = i + y_dir;
        let mut x = j + x_dir;

        loop {
            if y < 0 || y >= max_y || x < 0 || x >= max_x {
                break;
            }
            let c = config[y as usize][x as usize];
            if c == '#' {
                count += 1;
                break;
            }
            else if c == 'L' {
                break;
            }
            x += x_dir;
            y += y_dir;
        }
    }
    return count;
}

pub fn part_two() -> () {
	let lines = util::read_input(11);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut curr_config : Vec<Vec<char>> = list[..list.len()-1].iter().map(|&x| x.trim().chars().collect()).collect();

    loop {
        let mut next_config = curr_config.clone();
        let mut changed = false;
        for i in 0..curr_config.len() {
            for j in 0..curr_config[i].len() {
                let c = curr_config[i][j];
                if c == 'L' {
                    if get_occupied_neighbors_v2(&curr_config, i as i32, j as i32) == 0 {
                        next_config[i][j] = '#';
                        changed = true;
                    }
                }
                else if c == '#' {
                    if get_occupied_neighbors_v2(&curr_config, i as i32, j as i32) >= 5 {
                        next_config[i][j] = 'L';
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            let mut occupied_seats = 0;
            for row in curr_config {
                for c in row {
                    if c == '#' {
                        occupied_seats += 1;
                    }
                }
            }
            println!("Part 2: {}", occupied_seats);
            break;
        }
        curr_config = next_config;
    }
}