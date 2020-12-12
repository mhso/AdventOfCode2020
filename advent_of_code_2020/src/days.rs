pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;

pub fn get_solver_funcs() -> Vec<Vec<&'static dyn Fn() -> ()>> {
    let arr: Vec<Vec<&dyn Fn() -> ()>> = vec![
        vec![&day1::part_one, &day1::part_two],
        vec![&day2::part_one, &day2::part_two],
        vec![&day3::part_one, &day3::part_two],
        vec![&day4::part_one, &day4::part_two],
		vec![&day5::part_one, &day5::part_two],
		vec![&day6::part_one, &day6::part_two],
		vec![&day7::part_one, &day7::part_two],
		vec![&day8::part_one, &day8::part_two],
		vec![&day9::part_one, &day9::part_two],
		vec![&day10::part_one, &day10::part_two],
		vec![&day11::part_one, &day11::part_two]
    ];
    return arr
}