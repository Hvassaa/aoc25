use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("7.txt");
    let mut map: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == 'S' { '|' } else { c })
                .collect::<Vec<char>>()
        })
        .collect();

    let height = map.len();
    let width = map.first().unwrap().len();

    let mut sum = 0;

    (0..height - 1).for_each(|y| {
        (0..width).for_each(|x| {
            let val = map[y][x];
            if val == '|' {
                let under = map[y + 1][x];
                if under == '^' {
                    map[y + 1][x - 1] = '|';
                    map[y + 1][x + 1] = '|';
                    sum += 1;
                } else {
                    map[y + 1][x] = '|';
                }
            }
        });
    });

    println!("{}", sum);
}

pub fn second() {
    let lines = read_lines_of_file("7.txt");
    let mut map: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == 'S' {
                        1
                    } else if c == '^' {
                        -1
                    } else {
                        0
                    }
                })
                .collect::<Vec<i64>>()
        })
        .collect();

    let height = map.len();
    let width = map.first().unwrap().len();

    (0..height - 1).for_each(|y| {
        (0..width).for_each(|x| {
            let val = map[y][x];
            if val > 0 {
                let under = map[y + 1][x];
                if under == -1 {
                    map[y + 1][x + 1] = val + map[y + 1][x + 1];
                    map[y + 1][x - 1] = val + map[y + 1][x - 1];
                } else {
                    map[y + 1][x] = val;
                }
            }
        });
    });

    let sum: i64 = map[height - 2].iter().filter(|v| **v != -1).sum();

    println!("{}", sum);
}
