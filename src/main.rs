mod day1;
mod day2;

fn print_day(day: u8) {
    println!("-------------------------------------");
    println!("Day {day}\n");
}

fn main() {
    // Day 1
    if false {
        use day1::{solve_day1_part1, solve_day1_part2};

        print_day(1);
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

    // Day 2
    if true {
        use day2::{solve_day2_part1, solve_day2_part2};

        print_day(2);
        println!(
            "Part 1 test: {}",
            solve_day2_part1("./inputs/day2/test.txt")
        );
        println!(
            "Part 1 puzzle: {}",
            solve_day2_part1("./inputs/day2/input.txt")
        );
        println!(
            "Part 2 test: {}",
            solve_day2_part2("./inputs/day2/test.txt")
        );
        println!(
            "Part 2 puzzle: {}",
            solve_day2_part2("./inputs/day2/input.txt")
        );
    }
}
