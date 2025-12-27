use util::{Machine, Matrix};

use crate::util::read_file_to_lines;

mod util;

pub fn part1(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);
    let machines = lines.iter().map(Machine::from).collect::<Vec<Machine>>();

    let mut res = 0;

    for machine in machines {
        res += machine.button_presses();
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);
    let machines = lines.iter().map(Machine::from).collect::<Vec<Machine>>();

    let mut res = 0;

    for machine in machines {
        let matrix = Matrix::from_machine(&machine);

        // Now we can DFS over a much smaller solution space.
        let max = *machine.requirements.iter().max().unwrap() + 1;
        let mut min = usize::MAX;
        let mut values = vec![0; matrix.independents.len()];

        matrix.dfs(0, &mut values, &mut min, max);

        res += min;
    }

    res
}
