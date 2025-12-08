use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("6.txt");

    let ops: Vec<&str> = lines
        .iter()
        .rev()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .collect();

    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .rev()
        .skip(1)
        .rev()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let sum: u64 = ops
        .iter()
        .enumerate()
        .map(move |(idx, op)| {
            numbers
                .iter()
                .map(|e| e[idx])
                .reduce(|acc, e| match op {
                    &"*" => acc * e,
                    &"+" => acc + e,
                    _ => panic!(),
                })
                .unwrap()
        })
        .sum();

    println!("{}", sum);
}

pub fn second() {
    let lines = read_lines_of_file("6.txt");

    let ops: Vec<(usize, char)> = lines
        .iter()
        .rev()
        .nth(0)
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ' ')
        .collect();

    let lines: Vec<_> = lines.iter().rev().skip(1).rev().collect();

    let idxs: Vec<usize> = ops.iter().map(|e| e.0).collect();

    let mut slides_idxs = idxs.clone();
    slides_idxs.remove(0);
    slides_idxs.push(lines.first().unwrap().len() + 1);

    let column_takes: Vec<(usize, usize)> = idxs
        .iter()
        .zip(slides_idxs)
        .map(|(start, end)| (*start, end - start - 1))
        .collect();

    let s: Vec<Vec<Vec<char>>> = column_takes
        .iter()
        .map(|(skip, take)| {
            lines
                .iter()
                .map(|line| line.chars().skip(*skip).take(*take).collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect();

    // "temporary value created here" fucked me; spaghetti pants
    let cols_dims: Vec<Vec<u64>> = s
        .iter()
        .map(|cols| {
            cols.iter().fold(vec![], |acc, e| {
                let mut new_acc = acc.clone();
                if new_acc.is_empty() {
                    e.iter().for_each(|_| new_acc.push(String::from("")));
                }

                e.iter().enumerate().for_each(|(idx, c)| {
                    if *c != ' ' {
                        new_acc[idx] = format!("{}{}", new_acc[idx].to_string(), c.to_string());
                    }
                });

                new_acc
            })
        })
        .map(|a| {
            a.iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let sum: u64 = cols_dims
        .iter()
        .zip(ops)
        .map(|(cols, (_, op))| {
            let mut sum = 0;
            if op == '*' {
                sum = 1;
                cols.iter().for_each(|v| sum *= v);
            } else {
                cols.iter().for_each(|v| sum += v);
            }

            sum
        })
        .sum();

    println!("{}", sum);
}
