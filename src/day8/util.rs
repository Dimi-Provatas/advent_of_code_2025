use std::collections::HashMap;

pub fn get_distances_sorted(junctions: &Vec<Junction>) -> Vec<(usize, usize, usize)> {
    let mut distances: Vec<(usize, usize, usize)> = Vec::new();

    for current_junction in junctions {
        for other_junction in junctions {
            if current_junction.id == other_junction.id {
                continue;
            }

            let distance = {
                let dx = current_junction.x.abs_diff(other_junction.x).pow(2);
                let dy = current_junction.y.abs_diff(other_junction.y).pow(2);
                let dz = current_junction.z.abs_diff(other_junction.z).pow(2);

                dx + dy + dz
            };

            if current_junction.id > other_junction.id {
                distances.push((distance, current_junction.id, other_junction.id));
            }
        }
    }

    distances.sort_by_key(|s| s.0);

    distances
}

#[derive(Debug, Clone)]
pub struct Junction {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub fn parse_junctions(lines: Vec<String>) -> Vec<Junction> {
    let mut junctions = Vec::new();

    for (id, line) in lines.iter().enumerate() {
        let coords = line
            .splitn(3, ",")
            .map(|c| -> usize { c.parse().unwrap() })
            .collect::<Vec<usize>>();

        let x = coords[0];
        let y = coords[1];
        let z = coords[2];

        junctions.push(Junction { id, x, y, z })
    }

    junctions
}

#[derive(Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let parent = self.parent[x];
        if parent == x {
            x
        } else {
            let root = self.find(parent);
            self.parent[x] = root;
            root
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        self.parent[ra] = rb;
    }
}
