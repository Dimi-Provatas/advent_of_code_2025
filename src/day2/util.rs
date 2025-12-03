use std::fs;

pub fn is_invalid_part1(x: usize) -> bool {
    let num_str = x.to_string();

    let len = num_str.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let half_len = len / 2;
    num_str[0..half_len] == num_str[half_len..]
}

pub fn is_invalid_part2(x: usize) -> bool {
    let num_str = x.to_string();
    let len = num_str.len();
    let num_chars: Vec<char> = num_str.chars().collect();

    for substr_len in 2..=len {
        if !len.is_multiple_of(substr_len) {
            continue;
        }
        let size = len / substr_len;
        let mut ok = true;

        for i in (0..len).step_by(size) {
            let base = &num_chars[0..size];
            let segment = &num_chars[i..i + size];

            if segment != base {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }

    false
}

pub fn parse_input(filename: &str) -> Vec<(usize, usize)> {
    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut ranges = vec![];

    let input = input.split(',');
    for range in input {
        let (first, last) = range.split_once('-').expect("Could not split at '-'");
        let first: usize = first.trim().parse().expect("Could not parse first");
        let last: usize = last.trim().parse().expect("could not parse last");

        ranges.push((first, last));
    }

    ranges
}
