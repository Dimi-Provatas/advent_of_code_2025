use crate::util::read_file;

pub fn input_parser(filename: &str) -> Vec<(Action, u16)> {
    let input = read_file(filename);

    let mut sequence = Vec::new();

    for line in input {
        let (action, step) = line.split_at(1);

        let action = match action {
            "L" => Action::Subtract,
            "R" => Action::Add,
            _ => unreachable!(),
        };
        let step: u16 = step.parse().expect("Not a valid u8");

        sequence.push((action, step));
    }

    sequence
}

pub enum Action {
    Add,
    Subtract,
}
