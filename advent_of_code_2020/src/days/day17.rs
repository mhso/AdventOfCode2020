extern crate util;

fn get_active_neighbors(state: &Vec<Vec<Vec<char>>>, x: i32, y: i32, z: i32) -> Vec<char> {
    let mut neighbors = Vec::new();
    for i in z-1..z+2 {
        let i_u = i as usize;
        if i >= 0 && i < state.len() as i32 {
            for j in y-1..y+2 {
                let j_u = j as usize;
                if j >= 0 && j < state[i_u].len() as i32 {
                    for k in x-1..x+2 {
                        let k_u = k as usize;
                        if k >= 0 && k < state[i_u][j_u].len() as i32 {
                            if (i != z || j != y || k != x) && state[i_u][j_u][k_u] == '#' {
                                neighbors.push(state[i_u][j_u][k_u]);
                            }
                        }
                    }
                }
            }
        }
    }
    return neighbors;
}

fn count_active_cubes(state: &Vec<Vec<Vec<char>>>) -> u32 {
    let mut count = 0;
    for y_vec in state.iter() {
        for x_vec in y_vec.iter() {
            for cube in x_vec.iter() {
                if *cube == '#' {
                    count += 1;
                }
            }
        }
    }
    return count;
}

pub fn part_one() -> () {
	let lines = util::read_input(17);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let mut state: Vec<Vec<Vec<char>>> = Vec::new();

    let cycles = 6;
    let z_dims = 20;
    let y_dims = 20;
    let x_dims = 20;

    for z in 0..z_dims {
        let mut z_vec = Vec::new();
        for y in 0..y_dims {
            let mut y_vec = Vec::new();
            for _ in 0..x_dims {
                y_vec.push('.');
            }
            z_vec.push(y_vec);
        }
        state.push(z_vec);
    }

    for i in 0..list.len()-1 {
        let line = list[i].trim();

        let chars : Vec<char> = line.chars().collect();
        for j in 0..chars.len() {
            state[z_dims/2][(y_dims/2 - 4) + i][(x_dims/2 - 4) + j] = chars[j];
        }
    }

    for cycle in 0..cycles {
        let mut next_state = state.clone();
        for z in 0..state.len() {
            for y in 0..state[z].len() {
                for x in 0..state[y].len() {
                    let cube = state[z][y][x];
                    let neighbors = get_active_neighbors(&state, x as i32, y as i32, z as i32);
                    if cube == '#' && (neighbors.len() < 2 || neighbors.len() > 3) {
                        next_state[z][y][x] = '.';
                    }
                    else if cube == '.' && neighbors.len() == 3 {
                        next_state[z][y][x] = '#';
                    }
                }
            }
        }
        state = next_state.clone();
    }

    println!("Part 1: {}", count_active_cubes(&state));
}

fn get_active_neighbors_v2(state: &Vec<Vec<Vec<Vec<char>>>>, x: i32, y: i32, z: i32, w: i32) -> Vec<char> {
    let mut neighbors = Vec::new();
    for h in w-1..w+2 {
        let h_u = h as usize;
        if h >= 0 && h < state.len() as i32 {
            for i in z-1..z+2 {
                let i_u = i as usize;
                if i >= 0 && i < state[h_u].len() as i32 {
                    for j in y-1..y+2 {
                        let j_u = j as usize;
                        if j >= 0 && j < state[h_u][i_u].len() as i32 {
                            for k in x-1..x+2 {
                                let k_u = k as usize;
                                if k >= 0 && k < state[h_u][i_u][j_u].len() as i32 {
                                    if (h != w || i != z || j != y || k != x) && state[h_u][i_u][j_u][k_u] == '#' {
                                        neighbors.push(state[h_u][i_u][j_u][k_u]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return neighbors;
}

fn count_active_cubes_v2(state: &Vec<Vec<Vec<Vec<char>>>>) -> u32 {
    let mut count = 0;
    for z_vec in state.iter() {
        for y_vec in z_vec.iter() {
            for x_vec in y_vec.iter() {
                for cube in x_vec.iter() {
                    if *cube == '#' {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

pub fn part_two() -> () {
	let lines = util::read_input(17);
    let list: Vec<&str> = lines.split("\n").collect();
    
    let mut state: Vec<Vec<Vec<Vec<char>>>> = Vec::new();

    let cycles = 6;
    let w_dims = 20;
    let z_dims = 20;
    let y_dims = 20;
    let x_dims = 20;

    for w in 0..w_dims {
        let mut w_vec = Vec::new();
        for z in 0..z_dims {
            let mut z_vec = Vec::new();
            for y in 0..y_dims {
                let mut y_vec = Vec::new();
                for _ in 0..x_dims {
                    y_vec.push('.');
                }
                z_vec.push(y_vec);
            }
            w_vec.push(z_vec);
        }
        state.push(w_vec);
    }

    for i in 0..list.len()-1 {
        let line = list[i].trim();

        let chars : Vec<char> = line.chars().collect();
        for j in 0..chars.len() {
            state[w_dims/2][z_dims/2][(y_dims/2 - 4) + i][(x_dims/2 - 4) + j] = chars[j];
        }
    }

    for cycle in 0..cycles {
        let mut next_state = state.clone();
        for w in 0..state.len() {
            for z in 0..state[w].len() {
                for y in 0..state[z].len() {
                    for x in 0..state[y].len() {
                        let cube = state[w][z][y][x];
                        let neighbors = get_active_neighbors_v2(&state, x as i32, y as i32, z as i32, w as i32);
                        if cube == '#' && (neighbors.len() < 2 || neighbors.len() > 3) {
                            next_state[w][z][y][x] = '.';
                        }
                        else if cube == '.' && neighbors.len() == 3 {
                            next_state[w][z][y][x] = '#';
                        }
                    }
                }
            }
        }
        state = next_state.clone();
    }

    println!("Part 2: {}", count_active_cubes_v2(&state));
}