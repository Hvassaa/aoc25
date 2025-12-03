use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("3.txt");

    let sum: u64 = lines
        .iter()
        .map(|l| {
            let numbers: Vec<_> = l.chars().map(|c| c.to_digit(10)).collect();
            let size = numbers.len();

            let (first_max, idx) = numbers
                .iter()
                .zip(0..size)
                .take(size - 1)
                .max_by_key(|(val, _)| *val)
                .unwrap();

            let first_max = first_max.unwrap();
            let last_max = numbers.iter().skip(idx + 1).max().unwrap().unwrap();
            let result = format!("{}{}", first_max, last_max).parse::<u64>().unwrap();

            result
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {}
