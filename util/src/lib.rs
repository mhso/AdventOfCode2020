use std;
use std::io::Read;

pub fn read_input(day: u8) -> String {
    let mut file = std::fs::File::open(format!("input/day{}.txt", day)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}