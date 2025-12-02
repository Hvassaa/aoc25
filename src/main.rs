use std::fs;

mod day1;

fn main() {
    day1::first();
    day1::second();
}

fn read_lines_of_file(path: &str) -> Vec<String> {
    let file = fs::read_to_string(path).unwrap();
    let lines = file.lines();
    return lines.map(|line| line.to_string()).collect();
}
