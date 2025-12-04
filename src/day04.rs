#![allow(dead_code)]

const SENTINEL_CHAR: char = '.';

fn pad_grid(grid: String) -> Vec<Vec<char>> {
    let grid_unpadded: Vec<_> = grid
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let grid_unpadded_c = grid_unpadded[0].len();

    let mut grid_padded = grid_unpadded
        .into_iter()
        .map(|mut x| {
            x.insert(0, SENTINEL_CHAR);
            x.push(SENTINEL_CHAR);
            x
        })
        .collect::<Vec<_>>();
    grid_padded.insert(0, vec![SENTINEL_CHAR; grid_unpadded_c + 2]);
    grid_padded.push(vec![SENTINEL_CHAR; grid_unpadded_c + 2]);
    grid_padded
}

fn count_neighbors(grid: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> u64 {
    (grid[row_idx - 1][col_idx - 1] == '@') as u64
        + (grid[row_idx - 1][col_idx] == '@') as u64
        + (grid[row_idx - 1][col_idx + 1] == '@') as u64
        + (grid[row_idx][col_idx - 1] == '@') as u64
        + (grid[row_idx][col_idx] == '@') as u64
        + (grid[row_idx][col_idx + 1] == '@') as u64
        + (grid[row_idx + 1][col_idx - 1] == '@') as u64
        + (grid[row_idx + 1][col_idx] == '@') as u64
        + (grid[row_idx + 1][col_idx + 1] == '@') as u64
}

pub fn part1(input: String) -> u64 {
    let padded_input = pad_grid(input);
    // .into_iter()
    // .map(|x| x.into_iter().collect::<String>())
    // .collect::<Vec<String>>()
    // .join("\n");
    let mut num_forklift = 0;
    for row_idx in 1..padded_input.len() - 1 {
        for col_idx in 1..padded_input[0].len() - 1 {
            let test_char = padded_input[row_idx][col_idx];
            if test_char == '@' {
                if count_neighbors(&padded_input, row_idx, col_idx) <= 4 {
                    num_forklift += 1;
                }
            }
        }
    }
    num_forklift
}
