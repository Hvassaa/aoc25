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

            println!("initial: {:?}", numbers);
            let (a, _) = numbers.split_at(idx);
            println!("start: {:?}", a);

            println!("first {} at {}", first_max, idx);
            println!(
                "rest: {:?}",
                numbers.iter().skip(idx + 1).collect::<Vec<_>>()
            );
            let last_max = numbers.iter().skip(idx + 1).max().unwrap();
            println!("last: {:?}", last_max);
            let result = format!("{}{}", first_max, last_max).parse::<u64>().unwrap();

            println!("---");

            result
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {}
