#![allow(dead_code)]

use std::{cmp::Ordering, u64::MIN};

#[derive(Debug)]
struct Joltage((usize, u64));

impl Ord for Joltage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.1.cmp(&other.0.1)
    }
}

impl PartialOrd for Joltage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Joltage {
    fn eq(&self, other: &Self) -> bool {
        self.0.1 == other.0.1
    }
}

impl Eq for Joltage {}

fn find_max_joltage(bank: Vec<u64>) -> u64 {
    let mut current_max: u64 = MIN;
    for i in 0..bank.len() {
        for j in (i + 1)..bank.len() {
            let test_max = bank[i] * 10 + bank[j];
            if test_max > current_max {
                current_max = test_max;
            }
        }
    }
    // let mut heap = BinaryHeap::from(digits);
    // let (joltage1, joltage2) = (heap.pop().unwrap(), heap.pop().unwrap());
    // println!("{:?} {:?}", joltage1, joltage2);
    // match joltage1.0.0.cmp(&joltage2.0.0) {
    //     Ordering::Less => joltage1.0.1 * 10 + joltage2.0.1,
    //     Ordering::Greater => joltage2.0.1 * 10 + joltage1.0.1,
    //     Ordering::Equal => unreachable!(),
    // }
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
    banks.into_iter().map(|x| find_max_joltage(x)).sum()
}
