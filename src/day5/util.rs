pub fn parse_input(input: Vec<String>) -> (Vec<(usize, usize)>, Vec<usize>) {
    let split_at = input.iter().position(|s| s.is_empty()).unwrap();
    let (before, after) = input.split_at(split_at);

    let ids: Vec<usize> = after[1..]
        .to_vec()
        .iter()
        .map(|num| -> usize { num.parse().unwrap() })
        .collect();

    let mut ranges: Vec<(usize, usize)> = Vec::new();
    before.to_vec().iter().for_each(|line| {
        let (lower, upper) = line.split_once('-').unwrap();
        let lower: usize = lower.parse().unwrap();
        let upper: usize = upper.parse().unwrap();

        ranges.push((lower, upper));
    });

    (ranges, ids)
}
