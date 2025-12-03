#![allow(dead_code)]

use cached::proc_macro::cached;
use std::u64::MIN;

fn find_max_joltage_part1(bank: Vec<u64>) -> u64 {
    let mut current_max: u64 = MIN;
    for i in 0..bank.len() {
        for j in (i + 1)..bank.len() {
            let test_max = bank[i] * 10 + bank[j];
            current_max = current_max.max(test_max);
        }
    }
    current_max
}

pub fn part1(input: String) -> u64 {
    let banks: Vec<_> = input
        .split_whitespace()
        .map(|x| {
            x.chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as u64)
                .collect()
        })
        .collect();
    banks.into_iter().map(|x| find_max_joltage_part1(x)).sum()
}

const NUM_BATTERIES: usize = 12;

#[cached]
fn find_max_joltage_part2(bank: Vec<u64>, num_batteries_left: usize) -> u64 {
    if num_batteries_left == 0 {
        0
    } else if bank.len() == num_batteries_left {
        bank.iter().fold(0u64, |acc, d| acc * 10 + *d)
    } else {
        let (car, cdr) = (bank[0], bank[1..].to_vec());
        let take_head_digit: u64 = car * 10u64.pow((num_batteries_left - 1) as u32)
            + find_max_joltage_part2(cdr.clone(), num_batteries_left - 1);
        let discard_head_digit: u64 = find_max_joltage_part2(cdr.clone(), num_batteries_left);
        take_head_digit.max(discard_head_digit)
    }
}

pub fn part2(input: String) -> u64 {
    let banks: Vec<Vec<u64>> = input
        .split_whitespace()
        .map(|x| {
            x.chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as u64)
                .collect()
        })
        .collect();
    banks
        .iter()
        .map(|x| find_max_joltage_part2(x.clone(), NUM_BATTERIES))
        .sum()
}
