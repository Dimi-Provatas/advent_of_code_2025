use std::fs;

pub fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect()
}
