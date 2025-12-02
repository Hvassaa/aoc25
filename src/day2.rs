use crate::read_lines_of_file;

pub fn first() {
    let lines = read_lines_of_file("2.txt");
    let line = lines.first().unwrap();
    let ranges = line.split(',');

    let sum: u64 = ranges
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());
            start..end + 1
        })
        .map(|range| {
            range
                .map(|id| format!("{}", id))
                .filter(|id| id.chars().count() % 2 == 0)
                .map(|id| {
                    let mid = id.chars().count() / 2;
                    let (first, last) = id.split_at(mid);

                    let id_val = id.parse::<u64>().unwrap();
                    let matched = (first == last) as u64;

                    id_val * matched
                })
                .sum::<u64>()
        })
        .sum();

    println!("{:?}", sum);
}

pub fn second() {
    let lines = read_lines_of_file("2.txt");
    let line = lines.first().unwrap();
    let ranges = line.split(',');

    let sum: u64 = ranges
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());
            start..end + 1
        })
        .map(|range| {
            range
                .map(|id| format!("{}", id))
                .map(|id| {
                    let total_size = id.chars().count();
                    let found_repeating_pattern = (1..total_size).any(|chunk_size| {
                        if total_size % chunk_size != 0 {
                            return false;
                        }

                        let chars: Vec<_> = id.chars().collect::<Vec<_>>();
                        let chunks: Vec<_> = chars.chunks_exact(chunk_size).collect();

                        if chunks.len() < 2 {
                            return false;
                        }

                        let pattern_to_match = chunks.first().unwrap();
                        let all_match = chunks.iter().all(|pattern| pattern_to_match == pattern);

                        all_match
                    }) as u64;

                    let id_val = id.parse::<u64>().unwrap();

                    id_val * found_repeating_pattern
                })
                .sum::<u64>()
        })
        .sum();

    println!("{:?}", sum);
}
