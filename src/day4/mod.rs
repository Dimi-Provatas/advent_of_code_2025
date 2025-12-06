use crate::util::read_file;

use util::{MapSymbol, parse_char_in_input};

mod util;

pub fn part1(filename: &str) -> usize {
    let lines: Vec<Vec<MapSymbol>> = read_file(filename)
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| -> MapSymbol { parse_char_in_input(c) })
                .collect()
        })
        .collect();

    let mut res = 0;

    let line_max_idx = lines.len() - 1;
    let char_max_idx = lines[0].len() - 1;

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, char) in line.iter().enumerate() {
            if *char != MapSymbol::Scroll {
                continue;
            }

            // tl  tm  tr
            // ml char mr
            // bl  bm  br

            let tl = if line_idx == 0 || char_idx == 0 {
                MapSymbol::Oob
            } else {
                lines[line_idx - 1][char_idx - 1].clone()
            };

            let tm = if line_idx == 0 {
                MapSymbol::Oob
            } else {
                lines[line_idx - 1][char_idx].clone()
            };

            let tr = if line_idx == 0 || char_idx == char_max_idx {
                MapSymbol::Oob
            } else {
                lines[line_idx - 1][char_idx + 1].clone()
            };

            let ml = if char_idx == 0 {
                MapSymbol::Oob
            } else {
                lines[line_idx][char_idx - 1].clone()
            };

            let mr = if char_idx == char_max_idx {
                MapSymbol::Oob
            } else {
                lines[line_idx][char_idx + 1].clone()
            };

            let bl = if line_idx == line_max_idx || char_idx == 0 {
                MapSymbol::Oob
            } else {
                lines[line_idx + 1][char_idx - 1].clone()
            };

            let bm = if line_idx == line_max_idx {
                MapSymbol::Oob
            } else {
                lines[line_idx + 1][char_idx].clone()
            };

            let br = if line_idx == line_max_idx || char_idx == char_max_idx {
                MapSymbol::Oob
            } else {
                lines[line_idx + 1][char_idx + 1].clone()
            };

            let positions = [tl, tm, tr, ml, mr, bl, bm, br];
            let count = positions
                .iter()
                .filter(|x| **x == MapSymbol::Scroll)
                .count();

            if count < 4 {
                res += 1;
            }
        }
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let mut lines: Vec<Vec<MapSymbol>> = read_file(filename)
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| -> MapSymbol { parse_char_in_input(c) })
                .collect()
        })
        .collect();

    let mut res = 0;

    let line_max_idx = lines.len() - 1;
    let char_max_idx = lines[0].len() - 1;

    loop {
        let mut did_remove = false;

        for line_idx in 0..lines.len() {
            for (char_idx, char) in lines[line_idx].clone().iter().enumerate() {
                if *char != MapSymbol::Scroll {
                    continue;
                }

                // tl  tm  tr
                // ml char mr
                // bl  bm  br

                let tl = if line_idx == 0 || char_idx == 0 {
                    MapSymbol::Oob
                } else {
                    lines[line_idx - 1][char_idx - 1].clone()
                };

                let tm = if line_idx == 0 {
                    MapSymbol::Oob
                } else {
                    lines[line_idx - 1][char_idx].clone()
                };

                let tr = if line_idx == 0 || char_idx == char_max_idx {
                    MapSymbol::Oob
                } else {
                    lines[line_idx - 1][char_idx + 1].clone()
                };

                let ml = if char_idx == 0 {
                    MapSymbol::Oob
                } else {
                    lines[line_idx][char_idx - 1].clone()
                };

                let mr = if char_idx == char_max_idx {
                    MapSymbol::Oob
                } else {
                    lines[line_idx][char_idx + 1].clone()
                };

                let bl = if line_idx == line_max_idx || char_idx == 0 {
                    MapSymbol::Oob
                } else {
                    lines[line_idx + 1][char_idx - 1].clone()
                };

                let bm = if line_idx == line_max_idx {
                    MapSymbol::Oob
                } else {
                    lines[line_idx + 1][char_idx].clone()
                };

                let br = if line_idx == line_max_idx || char_idx == char_max_idx {
                    MapSymbol::Oob
                } else {
                    lines[line_idx + 1][char_idx + 1].clone()
                };

                let positions = [tl, tm, tr, ml, mr, bl, bm, br];
                let count = positions
                    .iter()
                    .filter(|x| **x == MapSymbol::Scroll)
                    .count();

                if count < 4 {
                    res += 1;
                    lines[line_idx][char_idx] = MapSymbol::Empty;
                    did_remove = true;
                }
            }
        }

        if !did_remove {
            break;
        }
    }

    res
}
