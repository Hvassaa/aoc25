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

    let mut map: HashMap<u64, Vec<Range<u64>>> = HashMap::new();

    let mut ys: Vec<u64> = ys.iter().map(|y| *y).collect();
    ys.sort();

    ys.iter().for_each(|y| {
        let mut match_y: Vec<u64> = b
            .iter()
            .filter(|(_, yp)| y == yp)
            .map(|(x, _)| *x)
            .collect();

        match_y.sort();
        let (_, _, qqq) = match_y.iter().fold((0, 0, vec![]), |acc, e| {
            let start = acc.0;
            let prev = acc.1;
            let ranges = acc.2;

            if start == 0 {
                // println!("{}", 1);
                return (*e, *e, ranges);
            }

            if prev == e + 1 {
                // println!("{}", 2);
                return (start, *e, ranges);
            }

            // println!("{}", 3);

            let mut ranges = ranges;
            ranges.push(start..prev);

            (*e, *e, ranges)
        });

        println!("{}, {:?}", y, qqq);
        map.insert(*y, qqq);
    });

    println!("running for {}", a.len().pow(2));

    let b = a
        .iter()
        .enumerate()
        .map(|(i, (x1, y1))| {
            a.iter()
                .enumerate()
                .filter(|(j, (x2, y2))| {
                    // println!("{}", (i + 1) * (j + 1));
                    let max_x = max(*x1, *x2);
                    let min_x = min(*x1, *x2);
                    let max_y = max(*y1, *y2);
                    let min_y = min(*y1, *y2);

                    let x_range = min_x..max_x + 1;
                    let y_range = min_y..max_y + 1;

                    y_range.clone().any(|y| {
                        let asd = map.get(&y);
                        if asd.is_none() {
                            return false;
                        }

                        let ranges = asd.unwrap();
                        x_range
                            .clone()
                            .all(|x| ranges.iter().filter(|r| r.contains(&x)).nth(1).is_some())
                    })
                })
                .map(|(_, (x2, y2))| (x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1))
                .max()
        })
        .max();

    // println!("{:?}", map);
}
