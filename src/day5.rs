use std::{
    cmp::{max_by, min_by},
    ops::Range,
};

use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("5.txt");

    let a: Vec<_> = lines
        .iter()
        .take_while(|l| !l.trim().is_empty())
        .map(|str_range| str_range.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .map(|(a, b)| a..b + 1)
        .collect();

    let b: Vec<u64> = lines
        .iter()
        .skip_while(|l| !l.trim().is_empty())
        .skip(1)
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    let c = b
        .iter()
        .filter(|id| a.iter().any(|range| range.contains(id)))
        .count();

    println!("{}", c);
}

fn merge(a: Vec<Range<u64>>) -> Vec<Range<u64>> {
    a.iter().fold(Vec::new(), |acc, e| {
        let (start, end) = (e.start, e.end);

        let overlap = acc.iter().enumerate().find(|(_, range)| {
            e.contains(&range.start)
                || e.contains(&range.end)
                || range.contains(&start)
                || range.contains(&end)
                || range.end == end // half-open interval: (0..1).contains(&1) = false !!!
        });

        let mut new_acc = acc.clone();

        match overlap {
            None => new_acc.push(e.clone()),
            Some((idx, overlapping_range)) => {
                let (existing_start, existing_end) =
                    (overlapping_range.start, overlapping_range.end);

                let merged_start = min_by(existing_start, start, |x, y| x.cmp(y));
                let merged_end = max_by(existing_end, end, |x, y| x.cmp(y));

                new_acc[idx] = merged_start..merged_end;
            }
        };

        new_acc
    })
}

pub fn second() {
    let lines = read_lines_of_file("5.txt");

    let mut count = 0;
    let mut prev_count = 1;
    let mut my_vec: Vec<Range<u64>> = lines
        .iter()
        .take_while(|l| !l.trim().is_empty())
        .map(|str_range| str_range.split_once("-").unwrap())
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .map(|(a, b)| a..b + 1)
        .collect();

    while count != prev_count {
        prev_count = count;
        my_vec = merge(my_vec);
        count = my_vec.len();
    }

    let sum: u64 = my_vec.iter().map(|range| range.end - range.start).sum();

    println!("{}", sum);
}
