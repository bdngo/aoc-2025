use std::{env, error::Error, fs};

mod day01;
mod day02;
mod day03;
mod day04;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let input = fs::read_to_string(fname)?;

    let solution = day04::part2(input);
    println!("{}", solution);

    Ok(())
}
