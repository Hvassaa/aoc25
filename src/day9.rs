use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("9.txt");

    let a: Vec<_> = lines
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    let b = a
        .iter()
        .map(|(x, y)| {
            a.iter()
                .map(|(nx, ny)| (x.abs_diff(*nx) + 1) * (y.abs_diff(*ny) + 1))
                .max()
        })
        .max()
        .unwrap()
        .unwrap();

    println!("{:?}", b);
}

pub fn second() {}
