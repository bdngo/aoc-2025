#![allow(dead_code)]
use crate::utils;

const MODULUS: i64 = 100;

pub fn part1(input: String) -> u64 {
    let mut dial: i64 = 50;
    let mut num_zeroes: u64 = 0;
    let with_direction: String = input.replace("R", "").replace("L", "-");
    let turns: Vec<i64> = utils::string_to_ints(with_direction);
    for turn in turns {
        dial = (dial + turn).rem_euclid(MODULUS);
        if dial == 0 {
            num_zeroes += 1;
        }
    }
    num_zeroes
}

pub fn part2(input: String) -> u64 {
    let mut dial: i64 = 50;
    let mut password: u64 = 0;
    let with_direction: String = input.replace("R", "").replace("L", "-");
    let turns: Vec<i64> = utils::string_to_ints(with_direction);
    for turn in turns {
        let num_crossings = if turn >= 0 {
            (dial + turn).div_euclid(MODULUS)
        } else {
            (dial - 1).div_euclid(MODULUS) - (dial + turn - 1).div_euclid(MODULUS) // magic formula from https://gist.github.com/icub3d/dc8ef4474449d327fda2336f3fe79df9
        };
        dial = (dial + turn).rem_euclid(MODULUS);
        password += num_crossings.unsigned_abs();
    }
    password
}
