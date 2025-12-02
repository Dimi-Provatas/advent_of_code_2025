use dial::Dial;
use util::{Action, input_parser};

mod dial;
mod util;

pub fn solve_day1_part1(input: &str) -> u16 {
    let action_sequence = input_parser(input);

    let mut dial = Dial::new();

    for (action, step) in action_sequence {
        match action {
            Action::Add => dial.add_part1(step),
            Action::Subtract => dial.subtract_part1(step),
        };
    }

    dial.solution
}

pub fn solve_day1_part2(input: &str) -> u16 {
    let action_sequence = input_parser(input);

    let mut dial = Dial::new();

    for (action, step) in action_sequence {
        match action {
            Action::Add => dial.add_part2(step),
            Action::Subtract => dial.subtract_part2(step),
        };
    }

    dial.solution
}
