#![allow(dead_code)]

use crate::utils;
use std::collections::BTreeSet;

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

pub fn part2(input: String) -> u64 {
    let id_ranges = input.split_once("\n\n").unwrap().0;
    let id_range_set = id_ranges
        .lines()
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| {
            let parsed_x: u64 = x.parse().unwrap();
            let parsed_y: u64 = y.parse().unwrap();
            (parsed_x..=parsed_y).collect::<BTreeSet<_>>()
        })
        .reduce(|acc, e| acc.union(&e).cloned().collect())
        .unwrap();
    id_range_set.len() as u64
}
