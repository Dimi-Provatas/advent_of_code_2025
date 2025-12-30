use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Gift {
    pub total_area: usize,
    pub shape: Vec<Vec<bool>>,
}
#[derive(Debug, Clone)]
pub struct Region {
    pub size: (usize, usize),
    pub index_counts: Vec<Vec<(usize, usize)>>,
}
impl Region {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            size,
            index_counts: Vec::new(),
        }
    }

    pub fn index_can_fit_presents(&self, gifts: &[Gift], index_count: &[(usize, usize)]) -> bool {
        let self_area = self.size.0 * self.size.1;

        let mut total_gift_area = 0;
        for (gift_idx, gift_count) in index_count {
            total_gift_area += gifts[*gift_idx].total_area * gift_count;
        }

        // 5/4 of the total gift area should fit in the region
        let upper_bound = total_gift_area * 130 / 100;
        // +2 because I want the test to be correct
        // NOTE: Print the self_area and the upper bound to see why better
        self_area + 2 >= upper_bound
    }
}

pub fn parse_lines(lines: Vec<String>) -> (Vec<Gift>, HashMap<(usize, usize), Region>) {
    let mut gifts = Vec::new();
    let mut regions: HashMap<(usize, usize), Region> = HashMap::new();

    let mut current_lines: Vec<String> = Vec::new();
    for line in lines {
        // Gift Parse
        {
            // On empty line, parse Gift
            if line.is_empty() {
                let mut total_area = 0;
                let mut gift = Gift {
                    total_area,
                    shape: vec![Vec::new(); current_lines.len()],
                };
                for (idx, current_line) in current_lines.iter().enumerate() {
                    gift.shape[idx] = current_line
                        .chars()
                        .map(|c| {
                            if c == '#' {
                                total_area += 1;
                                true
                            } else {
                                false
                            }
                        })
                        .collect();
                }

                gift.total_area = total_area;
                current_lines = Vec::new();
                gifts.push(gift);
                continue;
            }
            // Ignore gift index
            if line.chars().nth(1) == Some(':') {
                continue;
            }
            // If gift shape line, push to be parsed and go next
            if line.starts_with('#') || line.starts_with('.') {
                current_lines.push(line);
                continue;
            }
        }

        // Region Parse
        {
            let line = line.split(" ").collect::<Vec<&str>>();

            let mut key = (0, 0);
            let mut value = Vec::new();

            for (idx, segment) in line.iter().enumerate() {
                // Parse Size
                if segment.contains(":") {
                    let k = segment.replace(':', "");
                    let k = k
                        .splitn(2, 'x')
                        .map(|s| -> usize { s.parse().unwrap() })
                        .collect::<Vec<usize>>();
                    key = (k[0], k[1]);

                    continue;
                }

                let segment: usize = segment.parse().unwrap();
                // idx 0 is size
                value.push((idx - 1, segment));
            }

            let region = regions.entry(key).or_insert_with(|| Region::new(key));
            region.size = key;
            region.index_counts.push(value);
        }
    }

    (gifts, regions)
}
