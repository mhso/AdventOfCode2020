mod days;

fn get_solver_funcs() -> Vec<Vec<&'static dyn Fn() -> ()>> {
    let arr: Vec<Vec<&dyn Fn() -> ()>> = vec![
        vec![&days::day1::part_one, &days::day1::part_two],
        vec![&days::day2::part_one, &days::day2::part_two],
        vec![&days::day3::part_one, &days::day3::part_two],
        vec![&days::day4::part_one, &days::day4::part_two]
    ];
    return arr
}

fn main() {
    match std::env::args().nth(1) {
        Some(day_str) => {
            match std::env::args().nth(2) {
                Some(part_str) => {
                    let day : usize = day_str.parse().unwrap();
                    let part : usize = part_str.parse().unwrap();
                    let funcs = get_solver_funcs();
                    funcs[day - 1][part - 1]();
                },
                None => panic!("Need 'part' as input (1 or 2).")
            }
        },
        None => panic!("Need 'day' and 'part' as input.")
    }
}
