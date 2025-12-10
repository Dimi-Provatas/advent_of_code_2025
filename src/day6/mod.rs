use util::Calculation;

use crate::util::read_file_to_lines;

mod util;

pub fn part1(filename: &str) -> usize {
    let input = read_file_to_lines(filename);

    let lines: Vec<Vec<String>> = input
        .iter()
        .map(|line| {
            let mut res: Vec<String> = line.split(" ").map(String::from).collect();
            res.retain(|s| !s.is_empty());
            res
        })
        .collect();

    let mut numbers: Vec<Vec<usize>> = Vec::new();

    for line in lines.iter().take(lines.len() - 1) {
        for (number_idx, number) in line.iter().enumerate() {
            let number: usize = number.parse().unwrap();
            match numbers.get_mut(number_idx) {
                Some(col) => col.push(number),
                None => numbers.insert(number_idx, vec![number]),
            }
        }
    }

    let ops = lines[lines.len() - 1].clone();
    let ops: Vec<Calculation> = ops
        .iter()
        .map(|op| Calculation::parse(op.as_str()))
        .collect();

    assert_eq!(numbers.len(), ops.len());

    let mut res = 0;

    for (idx, nums) in numbers.iter().enumerate() {
        let mut line_res = 0;

        let op = ops[idx].clone();

        nums.iter().for_each(|num| {
            if op == Calculation::Multiply && line_res == 0 {
                line_res = 1;
            }

            match op {
                Calculation::Add => line_res += num,
                Calculation::Multiply => line_res *= num,
            }
        });

        res += line_res;
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);

    let mut nums_vec = vec![String::new(); lines[0].len() + 1];
    for line in lines.iter().take(lines.len() - 1) {
        let line: String = line.chars().rev().collect();
        for (char_idx, char) in line.chars().enumerate() {
            nums_vec[char_idx].push(char);
        }
    }
    let ops = lines[lines.len() - 1].clone();
    let mut ops: Vec<String> = ops.split(" ").map(String::from).collect();
    ops.retain(|s| !s.is_empty());
    ops.reverse();

    let mut res = 0;
    let mut current_nums = Vec::new();
    let mut current_op_idx = 0;

    for num in nums_vec {
        let num = num.trim();
        if !num.is_empty() {
            let num: usize = num.parse().unwrap();
            current_nums.push(num);
            continue;
        }

        let current_op = Calculation::parse(ops[current_op_idx].as_str());

        let mut group_res = 0;
        current_nums.iter().for_each(|num| {
            if current_op == Calculation::Multiply && group_res == 0 {
                group_res = 1;
            }

            match current_op {
                Calculation::Add => group_res += num,
                Calculation::Multiply => group_res *= num,
            }
        });

        res += group_res;
        current_nums = Vec::new();
        current_op_idx += 1;
    }

    res
}
