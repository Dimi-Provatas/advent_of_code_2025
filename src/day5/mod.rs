use crate::util::read_file;

use util::parse_input;

mod util;

pub fn part1(filename: &str) -> usize {
    let input = read_file(filename);
    let (ranges, ids) = parse_input(input);

    let mut res = 0;

    for id in ids {
        for (lower, upper) in &ranges {
            if (lower..=upper).contains(&&id) {
                res += 1;
                break;
            }
        }
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let input = read_file(filename);
    let (mut ranges_s, _) = parse_input(input);
    ranges_s.sort();

    let mut res = 0;
    let mut largest_id = 0;
    ranges_s.iter().for_each(|(lower, upper)| {
        let mut lower = *lower;
        let upper = *upper;

        if largest_id >= lower {
            lower = largest_id + 1;
        }

        if lower <= upper {
            res += upper - lower + 1; // plus 1 for inclusive range
        }

        largest_id = largest_id.max(upper);
    });

    res
}
