use std::{
    cmp::{max, min},
    collections::HashSet,
};

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

pub fn second() {
    let lines = read_lines_of_file("9.txt");

    let a: Vec<_> = lines
        .iter()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    let mut current_pos = a.iter().rev().nth(0).unwrap();

    let mut b: HashSet<(u64, u64)> = HashSet::new();

    a.iter().for_each(|next_pos| {
        let nx = next_pos.0;
        let ny = next_pos.1;
        let cx = current_pos.0;
        let cy = current_pos.1;

        let max_x = max(nx, cx);
        let min_x = min(nx, cx);
        let max_y = max(ny, cy);
        let min_y = min(ny, cy);

        let x_range = min_x..max_x + 1;
        let y_range = min_y..max_y + 1;

        if nx == cx {
            y_range.for_each(|y| {
                b.insert((nx, y));
            });
        } else if ny == cy {
            x_range.clone().for_each(|x| {
                b.insert((x, ny));
            });
        } else {
            panic!();
        }

        current_pos = next_pos;
    });

    println!("running for {}", a.len().pow(2));

    let b = a
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| {
            a.iter()
                .enumerate()
                .filter(|(j, (x2, y2))| {
                    println!("{}", (i + 1) * (j + 1));
                    let max_x = max(*x1, *x2);
                    let min_x = min(*x1, *x2);
                    let max_y = max(*y1, *y2);
                    let min_y = min(*y1, *y2);

                    let x_range = min_x..max_x + 1;
                    let y_range = min_y..max_y + 1;

                    let x_contained = x_range.clone().all(|x| {
                        let (x_to_find_1, y_to_find_1) = (x, y_range.start);
                        let (x_to_find_2, y_to_find_2) = (x, y_range.end - 1);
                        let under1 = b
                            .iter()
                            .any(|(x, y)| *x == x_to_find_1 && *y >= y_to_find_1);
                        let over1 = b
                            .iter()
                            .any(|(x, y)| *x == x_to_find_1 && *y <= y_to_find_1);

                        let under2 = b
                            .iter()
                            .any(|(x, y)| *x == x_to_find_2 && *y >= y_to_find_2);
                        let over2 = b
                            .iter()
                            .any(|(x, y)| *x == x_to_find_2 && *y <= y_to_find_2);
                        under1 && over1 && under2 && over2
                    });

                    let y_contained = y_range.clone().all(|y| {
                        let (x_to_find_1, y_to_find_1) = (x_range.start, y);
                        let (x_to_find_2, y_to_find_2) = (x_range.end - 1, y);
                        let under1 = b
                            .iter()
                            .any(|(x, y)| *y == y_to_find_1 && *x >= x_to_find_1);
                        let over1 = b
                            .iter()
                            .any(|(x, y)| *y == y_to_find_1 && *x <= x_to_find_1);

                        let under2 = b
                            .iter()
                            .any(|(x, y)| *y == y_to_find_2 && *x >= x_to_find_2);
                        let over2 = b
                            .iter()
                            .any(|(x, y)| *y == y_to_find_2 && *x <= x_to_find_2);

                        under1 && over1 && under2 && over2
                    });

                    x_contained && y_contained
                })
                .map(|(_, (x2, y2))| (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1))
                .max()
        })
        .max()
        .unwrap()
        .unwrap();

    println!("{}", b);
}
