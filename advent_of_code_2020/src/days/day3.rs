extern crate util;

pub fn part_one() -> () {
    let lines = util::read_input(3);
    let list: Vec<&str> = lines.split("\n").collect();

    let step_x = 3;
    let step_y = 1;
    let mut x = 0;
    let mut trees = 0;

    for y in (0..list.len()-1).step_by(step_y) {
        let line_chars: Vec<char> = list[y].trim().chars().collect();
        let c = line_chars[x];
        if c == '#' {
            trees += 1;
        }

        x += step_x;
        if x >= line_chars.len() {
            x = x - line_chars.len();
        }
    }

    println!("Part 1: {}", trees);
}

pub fn part_two() -> () {
    let lines = util::read_input(3);
    let list: Vec<&str> = lines.split("\n").collect();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees_mult = 1;

    for (step_x, step_y) in slopes {
        let mut trees = 0;
        let mut x = 0;
        for y in (0..list.len()-1).step_by(step_y) {
            let line_chars: Vec<char> = list[y].trim().chars().collect();
            println!("{}", list[y]);
            let c = line_chars[x];
            if c == '#' {
                trees += 1;
            }

            x += step_x;
            if x >= line_chars.len() {
                x = x - line_chars.len();
            }
        }
        println!("{}", trees);
        trees_mult *= trees;
    }

    println!("Part 2: {}", trees_mult);
}