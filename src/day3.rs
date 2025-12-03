use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("3.txt");

    let sum: u64 = lines
        .iter()
        .map(|l| {
            let numbers: Vec<u64> = l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            let size = numbers.len();

            let (idx, first_max) = numbers
                .iter()
                .enumerate()
                .take(size - 1)
                .rev()
                .max_by_key(|(_, val)| *val)
                .unwrap();

            let last_max = numbers.iter().skip(idx + 1).max().unwrap();
            let result = format!("{}{}", first_max, last_max).parse::<u64>().unwrap();

            result
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {
    let lines = read_lines_of_file("3.txt");

    let sum: u64 = lines
        .iter()
        .map(|l| {
            let numbers: Vec<u64> = l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            let v: Vec<u64> = [].to_vec();

            let (_, result_vec) = (0..12).fold((numbers, v), |acc, _| {
                let mut remaining = acc.0;
                let mut results = acc.1;
                let needs = 11 - results.len();
                let need_all = needs == remaining.len();

                remaining = if need_all {
                    let to_add = remaining[0];
                    results.push(to_add);

                    remaining[1..].to_vec()
                } else {
                    let (idx, val) = remaining
                        .iter()
                        .enumerate()
                        .rev()
                        .skip(needs)
                        .max_by_key(|(_, val)| **val)
                        .unwrap();

                    results.push(*val);
                    remaining[idx + 1..].to_vec()
                };

                (remaining, results)
            });

            result_vec
                .iter()
                .fold(String::from(""), |a, b| format!("{}{}", a, b))
                .parse::<u64>()
                .unwrap()
        })
        .sum();

    println!("{}", sum);
}
