pub fn part1(input: String) -> u64 {
    let mut dial: i64 = 50;
    let mut num_zeroes: u64 = 0;
    let with_direction: String = input.replace("R", "").replace("L", "-");
    let turns: Vec<i64> = with_direction
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    for turn in turns {
        dial = (dial + turn) % 100;
        if dial == 0 {
            num_zeroes += 1;
        }
    }
    num_zeroes
}
