use std::collections::{HashSet, VecDeque};

#[derive(Default, Clone, Debug)]
pub struct Machine {
    pub lights: usize,
    pub buttons: Vec<Vec<usize>>,
    pub requirements: Vec<usize>,
}
impl From<&String> for Machine {
    fn from(value: &String) -> Self {
        let segments = value.split(" ").collect::<Vec<&str>>();

        let mut lights = 0;
        let mut buttons = Vec::new();
        let mut requirements = Vec::new();

        for segment in segments {
            match segment.chars().next().unwrap() {
                '[' => {
                    let segment = segment.replace('[', "");
                    let segment = segment.replace(']', "");

                    for char in segment.chars().rev() {
                        lights = lights << 1 | if char == '#' { 1 } else { 0 };
                    }
                }
                '(' => {
                    let segment = segment.replace('(', "");
                    let segment = segment.replace(')', "");

                    let button = segment
                        .split(',')
                        .map(|s| -> usize { s.parse().unwrap() })
                        .collect::<Vec<usize>>();
                    buttons.push(button);
                }
                '{' => {
                    let segment = segment.replace('{', "");
                    let segment = segment.replace('}', "");

                    requirements = segment
                        .split(',')
                        .map(|s| -> usize { s.parse().unwrap() })
                        .collect::<Vec<usize>>();
                }
                _ => unreachable!(),
            }
        }

        Self {
            lights,
            buttons,
            requirements,
        }
    }
}
impl Machine {
    pub fn button_presses(&self) -> usize {
        let mut frontier = VecDeque::new();
        frontier.push_back((0, 0));

        let mut seen = HashSet::new();
        seen.insert(0);

        while let Some((lights, dist)) = frontier.pop_front() {
            if lights == self.lights {
                return dist;
            }

            for neighbour in self.buttons.iter() {
                let neighbour = neighbour.iter().fold(lights, |acc, n| acc ^ (1 << n));
                if seen.insert(neighbour) {
                    frontier.push_back((neighbour, dist + 1));
                }
            }
        }

        unreachable!()
    }
}

const EPSILON: f64 = 1e-9;
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
    pub dependents: Vec<usize>,
    pub independents: Vec<usize>,
}
impl Matrix {
    pub fn from_machine(machine: &Machine) -> Self {
        let rows = machine.requirements.len();
        let cols = machine.buttons.len();
        let mut data = vec![vec![0.0; cols + 1]; rows];

        for (idx, button) in machine.buttons.iter().enumerate() {
            for &light in button {
                if light < rows {
                    data[light][idx] = 1.0;
                }
            }
        }

        for (idx, &req) in machine.requirements.iter().enumerate() {
            data[idx][cols] = req as f64;
        }

        let mut matrix = Self {
            data,
            rows,
            cols,
            dependents: Vec::new(),
            independents: Vec::new(),
        };

        matrix.gaussian_elimination();

        matrix
    }

    fn gaussian_elimination(&mut self) {
        let mut pivot = 0;
        let mut col = 0;

        while pivot < self.rows && col < self.cols {
            let (best_row, best_value) = {
                let mut best_row: usize = 0;
                let mut best_value: f64 = 0.0;
                let mut found_any = false;

                for (idx, row) in self.data.iter().enumerate().skip(pivot) {
                    let val = row[col].abs();
                    if !found_any {
                        best_row = idx;
                        best_value = val;
                        found_any = true;
                    } else if val.partial_cmp(&best_value).unwrap().is_gt() {
                        best_row = idx;
                        best_value = val;
                    }
                }

                (best_row, best_value)
            };

            if best_value < EPSILON {
                self.independents.push(col);
                col += 1;
                continue;
            }

            self.data.swap(pivot, best_row);
            self.dependents.push(col);

            let pivot_value = self.data[pivot][col];
            for val in &mut self.data[pivot][col..=self.cols] {
                *val /= pivot_value;
            }

            for r in 0..self.rows {
                if r == pivot {
                    continue;
                }

                let factor = self.data[r][col];
                if factor.abs() > EPSILON {
                    let pivot_row = self.data[pivot][col..=self.cols].to_vec();
                    let pivot_len = pivot_row.len();
                    let mut pivot_idx = 0;

                    for j in col..=self.cols {
                        let val = &mut self.data[r][j];
                        let pivot_val = pivot_row[pivot_idx];
                        *val -= factor * pivot_val;
                        pivot_idx += 1;
                        if pivot_idx >= pivot_len {
                            break;
                        }
                    }
                }
            }

            pivot += 1;
            col += 1;
        }

        self.independents.extend(col..self.cols);
    }

    pub fn valid(&self, values: &[usize]) -> Option<usize> {
        let mut total = values.iter().sum::<usize>();

        for row in 0..self.dependents.len() {
            let mut val = self.data[row][self.cols];
            for (i, &col) in self.independents.iter().enumerate() {
                val -= self.data[row][col] * (values[i] as f64);
            }

            if val < -EPSILON {
                return None;
            }

            let rounded = val.round();
            if (val - rounded).abs() > EPSILON {
                return None;
            }

            total += rounded as usize;
        }

        Some(total)
    }

    pub fn dfs(&self, idx: usize, values: &mut [usize], min: &mut usize, max: usize) {
        if idx == self.independents.len() {
            if let Some(total) = self.valid(values) {
                *min = (*min).min(total);
            }
            return;
        }

        let total: usize = values[..idx].iter().sum();
        for val in 0..max {
            if total + val >= *min {
                break;
            }

            values[idx] = val;
            self.dfs(idx + 1, values, min, max);
        }
    }
}
