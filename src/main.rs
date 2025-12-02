mod day1;

fn main() {
    // Day 1
    if true {
        use day1::{solve_day1_part1, solve_day1_part2};

        println!("-------------------------------------");
        println!("Day 1\n");
        println!(
            "Part 1 test: {}",
            solve_day1_part1("./inputs/day1/test.txt")
        );
        println!(
            "Part 1 puzzle: {}",
            solve_day1_part1("./inputs/day1/input.txt")
        );
        println!(
            "Part 2 test: {}",
            solve_day1_part2("./inputs/day1/test.txt")
        );
        println!(
            "Part 2 puzzle: {}",
            solve_day1_part2("./inputs/day1/input.txt")
        );
    }
}
