use util::{is_invalid_part1, is_invalid_part2, parse_input};

mod util;

pub fn part1(filename: &str) -> usize {
    let ranges = parse_input(filename);

    let mut res = 0;

    for (range_low, range_high) in ranges {
        for id in range_low..=range_high {
            if is_invalid_part1(id) {
                res += id;
            }
        }
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let ranges = parse_input(filename);

    let mut res = 0;

    for (range_low, range_high) in ranges {
        for id in range_low..=range_high {
            if is_invalid_part2(id) {
                res += id;
            }
        }
    }

    res
}
