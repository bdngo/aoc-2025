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

pub fn part2(input: String) -> u64 {
    let mut dial: i64 = 50;
    let mut password: u64 = 0;
    let with_direction: String = input.replace("R", "").replace("L", "-");
    let turns: Vec<i64> = with_direction
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
    for turn in turns {
        let absolute_turns: u64 = turn.unsigned_abs();
        for _ in 0..absolute_turns {
            if turn < 0 {
                dial = (dial - 1) % 100;
            } else if turn > 0 {
                dial = (dial + 1) % 100;
            }
            if dial == 0 {
                password += 1;
            }
        }
    }
    password
}
