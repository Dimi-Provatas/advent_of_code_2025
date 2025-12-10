use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::read_file_to_lines;

pub fn part1(filename: &str) -> usize {
    let lines: Vec<Vec<String>> = read_file_to_lines(filename)
        .into_iter()
        .map(|line| -> Vec<String> { line.chars().map(String::from).collect() })
        .collect();

    let mut queue = VecDeque::new();
    for (idx, symbol) in lines[0].iter().enumerate() {
        if symbol == "S" {
            let start = (0usize, idx);
            queue.push_back(start);
            break;
        }
    }

    let mut res = 0;
    let mut seen = HashSet::new();
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);

        if current.0 == lines.len() - 1 {
            continue;
        }

        if lines[current.0][current.1] == "^" {
            queue.push_back((current.0 + 1, current.1 - 1));
            queue.push_back((current.0 + 1, current.1 + 1));
            res += 1
        } else {
            queue.push_back((current.0 + 1, current.1));
        }
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let lines: Vec<Vec<String>> = read_file_to_lines(filename)
        .into_iter()
        .map(|line| -> Vec<String> { line.chars().map(String::from).collect() })
        .collect();

    let mut start_point: (usize, usize) = (0, 0);
    for (idx, symbol) in lines[0].iter().enumerate() {
        if symbol == "S" {
            start_point.1 = idx;
            break;
        }
    }

    let mut timeline_cache: HashMap<(usize, usize), usize> = HashMap::new();

    fn calculate_timelines(
        lines: &Vec<Vec<String>>,
        cache: &mut HashMap<(usize, usize), usize>,
        line: usize,
        symbol: usize,
    ) -> usize {
        if line == lines.len() - 1 {
            cache.insert((line, symbol), 1);
            return 1;
        }

        if lines[line + 1][symbol] == "^" {
            let l = cache
                .get(&(line + 1, symbol - 1))
                .cloned()
                .unwrap_or_else(|| {
                    let res = calculate_timelines(lines, cache, line + 1, symbol - 1);
                    cache.insert((line + 1, symbol - 1), res);
                    res
                });

            let r = cache
                .get(&(line + 1, symbol + 1))
                .cloned()
                .unwrap_or_else(|| {
                    let res = calculate_timelines(lines, cache, line + 1, symbol + 1);
                    cache.insert((line + 1, symbol + 1), res);
                    res
                });

            l + r
        } else {
            cache.get(&(line + 1, symbol)).cloned().unwrap_or_else(|| {
                let res = calculate_timelines(lines, cache, line + 1, symbol);
                cache.insert((line + 1, symbol), res);
                res
            })
        }
    }

    calculate_timelines(&lines, &mut timeline_cache, start_point.0, start_point.1)
}
