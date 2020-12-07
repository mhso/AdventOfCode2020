mod days;

fn main() {
    match std::env::args().nth(1) {
        Some(day_str) => {
            match std::env::args().nth(2) {
                Some(part_str) => {
                    let day : usize = day_str.parse().unwrap();
                    let part : usize = part_str.parse().unwrap();
                    let funcs = days::get_solver_funcs();
                    funcs[day - 1][part - 1]();
                },
                None => panic!("Need 'part' as input (1 or 2).")
            }
        },
        None => panic!("Need 'day' and 'part' as input.")
    }
}
