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

pub fn part2(input: String) -> u64 {
    let (operands, operators) = input.trim_end().rsplit_once("\n").unwrap();
    let operand_matrix: Vec<_> = operands
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let operator_list: Vec<_> = operators.split_whitespace().collect();
    let num_equations = operator_list.len();

    let mut cephalopod_operands = transpose(&operand_matrix)
        .into_iter()
        .map(|x| x.into_iter().collect::<String>());
    let cephalopod_matrix: Vec<Vec<String>> = (0..num_equations)
        .map(|_| {
            cephalopod_operands
                .by_ref()
                .take_while(|x| !x.trim().is_empty())
                .collect::<Vec<_>>()
        })
        .collect();
    let cephalopod_matrix: Vec<Vec<u64>> = cephalopod_matrix
        .into_iter()
        .map(|x| {
            x.into_iter()
                .filter_map(|y| y.trim().parse().ok())
                .collect()
        })
        .collect();

    cephalopod_matrix
        .into_iter()
        .zip(operator_list)
        .filter_map(|(operands, operator)| match operator {
            "*" => operands.into_iter().reduce(|acc, e| acc * e),
            "+" => operands.into_iter().reduce(|acc, e| acc + e),
            _ => unreachable!(),
        })
        .sum()
}
