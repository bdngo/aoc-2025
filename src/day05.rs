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
    let mut id_ranges: Vec<_> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| {
            let parsed_x: u64 = x.parse().unwrap();
            let parsed_y: u64 = y.parse().unwrap();
            (parsed_x, parsed_y)
        })
        .collect();
    id_ranges.sort_by_key(|x| x.0);
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    for (range_start, range_end) in id_ranges {
        if let Some((_, last_end)) = merged_ranges.last_mut() {
            if range_start <= *last_end + 1 {
                *last_end = (*last_end).max(range_end);
            } else {
                merged_ranges.push((range_start, range_end));
            }
        } else {
            merged_ranges.push((range_start, range_end)); // one-time case to seed merged ranges
        }
    }
    merged_ranges.into_iter().map(|(x, y)| y - x + 1).sum()
}
