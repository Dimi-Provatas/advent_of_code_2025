use crate::util::read_file_to_lines;

pub fn part1(filename: &str) -> usize {
    let input = read_file_to_lines(filename);

    let mut res = 0;

    for line in input {
        let chars: Vec<u8> = line
            .chars()
            .map(|c| -> u8 { c.to_digit(10).unwrap() as u8 })
            .collect();
        let char_len = chars.len();

        let mut l_larger = 0;
        let mut l_idx: usize = 0;
        let mut r_larger = 0;

        for (idx, current_char) in chars.iter().enumerate().take(char_len - 1) {
            let current_char = *current_char;
            if current_char == 9 {
                l_idx = idx;
                l_larger = 9;
                break;
            }

            if current_char > l_larger {
                l_larger = current_char;
                l_idx = idx;
            }
        }

        for current_char in chars.iter().take(char_len).skip(l_idx + 1) {
            let current_char = *current_char;
            if current_char == 9 {
                r_larger = 9;
                break;
            }

            if current_char > r_larger {
                r_larger = current_char;
            }
        }

        let res_str = format!("{l_larger}{r_larger}");
        let parsed_res: usize = res_str.parse().unwrap();
        res += parsed_res;
    }

    res
}

pub fn part2(filename: &str) -> usize {
    let input = read_file_to_lines(filename);

    let mut res = 0;

    for line in input {
        let mut chars: Vec<u8> = line
            .chars()
            .map(|c| -> u8 { c.to_digit(10).unwrap() as u8 })
            .collect();
        let mut line_res = 0;

        for idx in 0..11 {
            let slice = &chars[..chars.len() + idx - 11];

            let max = slice.iter().max().cloned().unwrap();
            let max_idx = slice.iter().position(|&c| c == max).unwrap();

            chars = chars[max_idx + 1..].to_vec();
            line_res = line_res * 10 + max as usize;
        }

        let final_max = chars.iter().max().cloned().unwrap();
        line_res = line_res * 10 + final_max as usize;

        res += line_res;
    }

    res
}
