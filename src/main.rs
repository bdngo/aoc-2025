use std::{env, error::Error, fs};

pub mod day01;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let input = fs::read_to_string(fname)?;

    let solution = day01::part2(input);
    println!("{}", solution);

    Ok(())
}
