use std::sync::OnceLock;

use util::parse_lines;

use crate::util::read_file_to_lines;

mod util;

static HAS_PRINTED: OnceLock<bool> = OnceLock::new();

pub fn part1(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);
    let (gifts, regions) = parse_lines(lines);

    let mut res = 0;

    for (_, region) in regions {
        for index_count in &region.index_counts {
            if region.index_can_fit_presents(&gifts, index_count) {
                res += 1;
            }
        }
    }

    res
}

pub fn part2(_filename: &str) -> usize {
    if HAS_PRINTED.get().is_none() {
        println!("\nDay 12 has no part 2");
        HAS_PRINTED.set(true).unwrap();
    }
    0
}
