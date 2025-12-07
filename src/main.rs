mod util;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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
}

fn main() {
    // solve_day!(day1);

    // solve_day!(day2);

    // solve_day!(day3);

    // solve_day!(day4);

    // solve_day!(day5);

    solve_day!(day6);
}
