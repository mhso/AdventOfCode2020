extern crate util;

fn move_point(x: i32, y: i32, value: i32, dir: i32) -> (i32, i32) {
    return match dir {
        270 => (x, y + value), // North
        90 => (x, y - value), // South
        0 => (x + value, y), // East
        180 => (x - value, y), // West
        _ => (x, y)
    }
}

fn get_direction(c: char) -> i32 {
    match c {
        'N' => 270,
        'S' => 90,
        'E' => 0,
        'W' => 180,
        _ => 0
    }
}

pub fn part_one() -> () {
	let lines = util::read_input(12);
    let list: Vec<&str> = lines.split("\n").collect();

    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        let c = line.as_bytes()[0] as char;
        let split : Vec<&str> = line.split(c).collect();
        let joined = split.join("");
        let value : i32 = joined.parse().unwrap();

        if c == 'L' {
            direction = ((360 - value) + direction) % 360;
        }
        else if c == 'R' {
            direction = (direction + value) % 360;
        }
        else {
            let (new_x, new_y) = match c {
                'F' => move_point(x, y, value, direction),
                _ => move_point(x, y, value, get_direction(c))
            };
            x = new_x;
            y = new_y;
        }
    }
    let manhattan = x.abs() + y.abs();
    println!("Part 1: {}", manhattan);
}

fn rotate_waypoint(x: i32, y: i32, deg: i32) -> (i32, i32) {
    let normed_deg = if deg < 0 { 360 + deg } else { deg };
    return match normed_deg {
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => (x, y)
    };
}

fn move_ship_with_waypoint(ship_x: i32, ship_y: i32, way_x: i32, way_y: i32, factor: i32) -> (i32, i32) {
    return (ship_x + way_x * factor, ship_y + way_y * factor)
}

pub fn part_two() -> () {
	let lines = util::read_input(12);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;
    let mut ship_x = 0;
    let mut ship_y = 0;

    for i in 0..list.len()-1 {
        let line = list[i].trim();
        let c = line.as_bytes()[0] as char;
        let split : Vec<&str> = line.split(c).collect();
        let joined = split.join("");
        let value : i32 = joined.parse().unwrap();

        if c == 'F' {
            let (new_x, new_y) = move_ship_with_waypoint(ship_x, ship_y, waypoint_x, waypoint_y, value);
            ship_x = new_x;
            ship_y = new_y;
        }
        else {
            let (new_x, new_y) = match c {
                'L' => rotate_waypoint(waypoint_x, waypoint_y, -value),
                'R' => rotate_waypoint(waypoint_x, waypoint_y, value),
                _   => move_point(waypoint_x, waypoint_y, value, get_direction(c))
            };
            waypoint_x = new_x;
            waypoint_y = new_y;
        }
    }
    let manhattan = ship_x.abs() + ship_y.abs();
    println!("Part 2: {}", manhattan);
}