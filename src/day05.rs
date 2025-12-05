#![allow(dead_code)]

use crate::utils;

pub fn part1(input: String) -> u64 {
    let (id_ranges, ids) = input.split_once("\n\n").unwrap();
    let id_ranges: Vec<_> = id_ranges
        .lines()
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| {
            let parsed_x: u64 = x.parse().unwrap();
            let parsed_y: u64 = y.parse().unwrap();
            parsed_x..=parsed_y
        })
        .collect();
    let ids: Vec<u64> = utils::string_to_ints(ids.to_string());
    ids.into_iter()
        .map(|x| id_ranges.iter().any(|y| y.contains(&x)) as u64)
        .sum()
}
