use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::Range,
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

    let ys: HashSet<u64> = b.iter().map(|(_, y)| *y).collect();

    let mut ys: Vec<u64> = ys.iter().map(|y| *y).collect();
    ys.sort();
    let mut mapa: HashMap<u64, Vec<Range<u64>>> = HashMap::new();

    println!("total {}", ys.len());
    ys.iter().enumerate().for_each(|(i, y)| {
        println!("handling {}", i);
        let mut match_y: Vec<u64> = b
            .iter()
            .filter(|(_, yp)| y == yp)
            .map(|(x, _)| *x)
            .collect();

        match_y.sort();

        let mut start: Option<u64> = Option::None;
        let mut ranges: Vec<Range<u64>> = vec![];

        (1..match_y.len()).for_each(|i| {
            let prev = match_y[i - 1];
            let current = match_y[i];

            if start.is_none() {
                start = Option::Some(prev);
            }

            if prev == current - 1 && i < match_y.len() - 1 {
                return;
            }
            ranges.push(start.unwrap()..current + 1);
            start = Option::None;
        });

        mapa.insert(*y, ranges);
    });

    println!("running for {}", a.len().pow(2));

    let b = a
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| {
            a.iter()
                .filter(|(x2, y2)| x1 != x2 && y1 != y2)
                .enumerate()
                .filter(|(j, (x2, y2))| {
                    println!("{}", (i + 1) * (j + 1));
                    let max_x = max(*x1, *x2);
                    let min_x = min(*x1, *x2);
                    let max_y = max(*y1, *y2);
                    let min_y = min(*y1, *y2);

                    let mut x_range = min_x..max_x + 1;
                    let mut y_range = min_y..max_y + 1;
                    let top = {
                        let ranges = mapa.get(&y_range.start);
                        match ranges {
                            None => false,
                            Some(ranges) => {
                                x_range.all(|x| ranges.iter().any(|range| range.contains(&x)))
                            }
                        }
                    };

                    let bot = {
                        let ranges = mapa.get(&(y_range.end));
                        match ranges {
                            None => false,
                            Some(ranges) => {
                                x_range.all(|x| ranges.iter().any(|range| range.contains(&x)))
                            }
                        }
                    };

                    let left_right = {
                        let xs = x_range.start;
                        let xe = x_range.end;
                        y_range.all(|y| {
                            let ranges = mapa.get(&y);

                            match ranges {
                                None => false,
                                Some(ranges) => {
                                    ranges.iter().any(|range| range.contains(&xs))
                                        && ranges.iter().any(|range| range.contains(&xe))
                                }
                            }
                        })
                    };

                    top && bot && left_right
                })
                // .inspect(|(_, qqq)| println!("{:?} {:?}", (x1, y1), qqq))
                .map(|(_, (x2, y2))| (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1))
                .max()
        })
        .max()
        .unwrap()
        .unwrap();

    println!("{:?}", b);
}
