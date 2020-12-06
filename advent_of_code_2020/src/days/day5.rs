extern crate util;

fn binary_search(min : u32, max : u32, chars : Vec<char>, index : usize, lower_char : char) -> u32 {
    if max == min {
        return min;
    }
    else {
        if chars[index] == lower_char {
            return binary_search(min, max - (max - min) / 2 - 1, chars, index + 1, lower_char)
        }
        else {
            return binary_search(min + (max - min) / 2 + 1, max, chars, index + 1, lower_char)
        }
    }
}

pub fn part_one() -> () {
    let lines = util::read_input(5);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut highest_id = 0;

    for i in 0..list.len()-1 {
        let line = list[i].trim();

        let row = binary_search(0, 127, line[..line.len()-3].chars().collect(), 0, 'F');
        let col = binary_search(0, 7, line[line.len()-3..].chars().collect(), 0, 'L');

        let seat_id = row * 8 + col;
        if seat_id > highest_id {
            highest_id = seat_id;
        }
    }

    println!("Part 1: {}", highest_id);
}

pub fn part_two() -> () {
    let lines = util::read_input(5);
    let list: Vec<&str> = lines.split("\n").collect();
    let mut seats : Vec<u32> = Vec::new();

    for i in 0..list.len()-1 {
        let line = list[i].trim();

        let row = binary_search(0, 127, line[..line.len()-3].chars().collect(), 0, 'F');
        let col = binary_search(0, 7, line[line.len()-3..].chars().collect(), 0, 'L');

        let seat_id = row * 8 + col;
        seats.push(seat_id);
    }

    seats.sort();

    let mut prev = seats[0];
    for seat_id in seats {
        if seat_id - prev == 2 {
            println!("Part 2: {}", seat_id-1);
            break;
        }
        prev = seat_id;
    }
}