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

    let width = map.first().unwrap().len();
    let height = map.len();

    let mut sum = 0;

    (0..height).for_each(|y| {
        (0..width).for_each(|x| {
            if map[y][x] == '@' {
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
                    .map(|(ny, nx)| map[((y as isize) + ny) as usize][((x as isize) + nx) as usize])
                    .filter(|a| *a == '@')
                    .count();

                if hits < 4 {
                    sum += 1;
                }
            }
        })
    });

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

    let width = map.first().unwrap().len();
    let height = map.len();

    let mut sum = 0;
    let mut prev_sum = -1;

    while sum != prev_sum {
        prev_sum = sum;

        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                if map[y][x] == '@' {
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
                            map[((y as isize) + ny) as usize][((x as isize) + nx) as usize]
                        })
                        .filter(|a| *a == '@')
                        .count();

                    if hits < 4 {
                        map[y][x] = 'x';
                        sum += 1;
                    }
                }
            })
        });
    }

    println!("{}", sum);
}
