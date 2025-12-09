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

    let max_x = *b.iter().map(|(x, _)| x).max().unwrap() as usize;
    let max_y = *b.iter().map(|(_, y)| y).max().unwrap() as usize;

    let mut map: Vec<Vec<u64>> = vec![vec![0; max_x + 1]; max_y + 1];
    b.iter()
        .for_each(|(x, y)| map[*y as usize][*x as usize] = 1);

    let mut to_inside = 2;
    (0..max_y).for_each(|y| {
        let mut inside = false;
        (0..max_x).for_each(|x| {
            let v = map[y][x];
            let next_v = map[y].get(x + 1);

            let next_is_0 = next_v.is_some_and(|q| *q == 0) || next_v.is_none();

            if v == 1 && next_is_0 {
                inside = !inside;
            } else if inside {
                map[y][x] = 2;
                to_inside += 1;
            }
        });
    });

    map.iter().for_each(|l| println!("{:?}", l));

    let b = a
        .iter()
        .map(|(x1, y1)| {
            a.iter()
                .filter(|(x2, y2)| {
                    let max_x = max(*x1, *x2);
                    let min_x = min(*x1, *x2);
                    let max_y = max(*y1, *y2);
                    let min_y = min(*y1, *y2);

                    let x_range = min_x..max_x + 1;
                    let y_range = min_y..max_y + 1;

                    let x_contained = x_range.clone().all(|x| {
                        map[y_range.start as usize][x as usize] > 0
                            && map[(y_range.end - 1) as usize][x as usize] > 0
                    });

                    let y_contained = y_range.clone().all(|y| {
                        map[y as usize][x_range.start as usize] > 0
                            && map[y as usize][(x_range.end - 1) as usize] > 0
                    });

                    x_contained && y_contained
                })
                .map(|(x2, y2)| (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1))
                .max()
        })
        .max()
        .unwrap()
        .unwrap();

    println!("{}", b);

    // a.iter().skip(1).for_each(|next_pos| {
    //     let nx = next_pos.0;
    //     let ny = next_pos.1;
    //     let cx = current_pos.0;
    //     let cy = current_pos.1;
    //
    //     let max_x = max(nx, cx);
    //     let min_x = min(nx, cx);
    //     let max_y = max(ny, cy);
    //     let min_y = min(ny, cy);
    //
    //     let x_range = min_x..max_x + 1;
    //     let y_range = min_y..max_y + 1;
    //
    //     x_range.clone().for_each(|x| {
    //         b.insert((x, y_range.start));
    //         b.insert((x, y_range.end));
    //     });
    //
    //     y_range.for_each(|y| {
    //         b.insert((x_range.start, y));
    //         b.insert((x_range.end, y));
    //     });
    //
    //     current_pos = next_pos;
    // });
}
