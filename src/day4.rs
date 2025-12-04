use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("4.txt");

    let mut lines: Vec<String> = lines.iter().rev().map(|l| format!(".{}.", l)).collect();

    let line_size = lines.first().unwrap().len();
    let empty_line = (0..line_size)
        .map(|_| ".")
        .fold(String::from(""), |acc, e| format!("{}{}", acc, e));

    lines.insert(0, empty_line.clone());
    lines.push(empty_line);

    let map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let sum: usize = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let a: usize = line
                .iter()
                .enumerate()
                .map({
                    let value = map.clone();

                    move |(x, val)| {
                        if *val != '@' {
                            return 0;
                        }

                        let a: [(isize, isize); 8] = [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ];

                        let hits = a
                            .iter()
                            .map(|(ny, nx)| {
                                value[((y as isize) + ny) as usize][((x as isize) + nx) as usize]
                            })
                            .filter(|a| *a == '@')
                            .count();

                        (hits < 4) as usize
                    }
                })
                .sum();
            a
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {
    let lines = read_lines_of_file("4.txt");

    let mut lines: Vec<String> = lines.iter().rev().map(|l| format!(".{}.", l)).collect();

    let line_size = lines.first().unwrap().len();
    let empty_line = (0..line_size)
        .map(|_| ".")
        .fold(String::from(""), |acc, e| format!("{}{}", acc, e));

    lines.insert(0, empty_line.clone());
    lines.push(empty_line);

    let mut map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let sum: usize = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let a: usize = line
                .iter()
                .enumerate()
                .map({
                    let value = map.clone();

                    move |(x, val)| {
                        if *val != '@' {
                            return 0;
                        }

                        let a: [(isize, isize); 8] = [
                            (-1, -1),
                            (-1, 0),
                            (-1, 1),
                            (0, -1),
                            (0, 1),
                            (1, -1),
                            (1, 0),
                            (1, 1),
                        ];

                        let hits = a
                            .iter()
                            .map(|(ny, nx)| {
                                value[((y as isize) + ny) as usize][((x as isize) + nx) as usize]
                            })
                            .filter(|a| *a == '@')
                            .count();

                        (hits < 4) as usize
                    }
                })
                .sum();
            a
        })
        .sum();

    println!("{}", sum);
}
