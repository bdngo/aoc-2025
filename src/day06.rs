#![allow(dead_code)]

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..matrix[0].len())
        .map(|col| matrix.iter().map(|row| row[col].clone()).collect())
        .collect()
}

pub fn part1(input: String) -> u64 {
    let (operands, operators) = input.trim().rsplit_once("\n").unwrap();
    let operand_matrix: Vec<Vec<u64>> = operands
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|y| y.parse().ok())
                .collect()
        })
        .collect();
    let operator_list: Vec<_> = operators.split_whitespace().collect();
    let transposed_operands = transpose(&operand_matrix);
    transposed_operands
        .into_iter()
        .zip(operator_list)
        .filter_map(|(operands, operator)| match operator {
            "*" => operands.into_iter().reduce(|acc, e| acc * e),
            "+" => operands.into_iter().reduce(|acc, e| acc + e),
            _ => unreachable!(),
        })
        .sum()
}
