mod util;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn print_day(day: &str) {
    println!("-------------------------------------");
    let day = day.replace("day", "");
    println!("Day {day}\n");
}

macro_rules! solve_day {
    ($day:ident) => {
        let day_str = stringify!($day);
        let test_path = format!("./inputs/{day_str}/test.txt");
        let puzzle_path = format!("./inputs/{day_str}/input.txt");

        print_day(day_str);
        println!("Part 1 test:   {}", $day::part1(&test_path));
        println!("Part 1 puzzle: {}", $day::part1(&puzzle_path));
        println!("Part 2 test:   {}", $day::part2(&test_path));
        println!("Part 2 puzzle: {}", $day::part2(&puzzle_path));
    };

    ($day:ident, separate_tests) => {
        let day_str = stringify!($day);
        let puzzle_path = format!("./inputs/{day_str}/input.txt");

        print_day(day_str);
        let test_path = format!("./inputs/{day_str}/test_1.txt");
        println!("Part 1 test:   {}", $day::part1(&test_path));
        println!("Part 1 puzzle: {}", $day::part1(&puzzle_path));
        let test_path = format!("./inputs/{day_str}/test_2.txt");
        println!("Part 2 test:   {}", $day::part2(&test_path));
        println!("Part 2 puzzle: {}", $day::part2(&puzzle_path));
    };
}

fn main() {
    // solve_day!(day1);

    // solve_day!(day2);

    // solve_day!(day3);

    // solve_day!(day4);

    // solve_day!(day5);

    // solve_day!(day6);

    // solve_day!(day7);

    // solve_day!(day8);

    // solve_day!(day9);

    // solve_day!(day10);

    // NOTE: Of course day 11 has 2 separate test inputs
    solve_day!(day11, separate_tests);
}
