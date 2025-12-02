#![allow(dead_code)]

fn is_invalid_id_part1(id: u64) -> Option<u64> {
    let id_string = id.to_string();
    if !id_string.len().is_multiple_of(2) {
        None
    } else {
        let (f_half, b_half) = id_string.split_at(id_string.len() / 2);
        if f_half == b_half { Some(id) } else { None }
    }
}

pub fn part1(input: String) -> u64 {
    let ranges: Vec<_> = input
        .trim()
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| {
            let parsed_x: u64 = x.parse().unwrap();
            let parsed_y: u64 = y.parse().unwrap();
            parsed_x..=parsed_y
        })
        .collect();
    let mut invalid_id_sum: u64 = 0;
    for range in ranges {
        invalid_id_sum += range.filter_map(|id| is_invalid_id_part1(id)).sum::<u64>();
    }
    invalid_id_sum
}

fn is_invalid_id_part2(id: u64) -> Option<u64> {
    let id_string = id.to_string();
    let id_len = id_string.len();
    for chunk_length in 1..=(id_len / 2) {
        if id_len.is_multiple_of(chunk_length) {
            let test_subseq = &id_string[..chunk_length];
            if test_subseq.repeat(id_len / chunk_length) == id_string {
                return Some(id);
            }
        }
    }
    None
}

pub fn part2(input: String) -> u64 {
    let ranges: Vec<_> = input
        .trim()
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| {
            let parsed_x: u64 = x.parse().unwrap();
            let parsed_y: u64 = y.parse().unwrap();
            parsed_x..=parsed_y
        })
        .collect();
    let mut invalid_id_sum: u64 = 0;
    for range in ranges {
        let invalid_ids = range.filter_map(|id| is_invalid_id_part2(id));
        invalid_id_sum += invalid_ids.sum::<u64>();
    }
    invalid_id_sum
}
