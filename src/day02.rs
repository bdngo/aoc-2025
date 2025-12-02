fn is_invalid_id(id: u64) -> Option<u64> {
    let id_string = id.to_string();
    if !id_string.len().is_multiple_of(2) {
        None
    } else {
        let (f_half, b_half) = id_string.split_at(id_string.len() / 2);
        if f_half == b_half { Some(id) } else { None }
    }
}

pub fn part1(input: String) -> u64 {
    let ranges: Vec<_> = input
        .trim()
        .split(",")
        .map(|x| x.split_once("-").unwrap())
        .map(|(x, y)| {
            let parsed_x: u64 = x.parse().unwrap();
            let parsed_y: u64 = y.parse().unwrap();
            parsed_x..=parsed_y
        })
        .collect();
    let mut invalid_id_sum: u64 = 0;
    for range in ranges {
        invalid_id_sum += range.filter_map(|id| is_invalid_id(id)).sum::<u64>();
    }
    invalid_id_sum
}
