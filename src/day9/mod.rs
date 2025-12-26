use util::{Point, Polygon, parse_points};

use crate::util::read_file_to_lines;

mod util;

pub fn part1(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);

    let points = parse_points(lines);

    let mut all_areas: Vec<usize> = Vec::new();

    for i in 0..points.len() {
        let point_a = points[i].clone();

        for j in i..points.len() {
            let point_b = points[j].clone();

            let x = point_a.x.abs_diff(point_b.x) + 1;
            let y = point_a.y.abs_diff(point_b.y) + 1;

            let area = x * y;
            all_areas.push(area);
        }
    }

    all_areas.sort_unstable();
    all_areas.reverse();
    all_areas[0]
}

pub fn part2(filename: &str) -> usize {
    let lines = read_file_to_lines(filename);

    let points = parse_points(lines);
    let polygon = Polygon::new(points.clone());

    let mut all_areas: Vec<usize> = Vec::new();

    for i in 0..points.len() {
        let point_a = points[i].clone();

        for j in i..points.len() {
            let point_b = points[j].clone();

            if point_b == point_a {
                continue;
            }

            // Check points are in the polygon
            {
                let other_point_1 = Point {
                    x: point_a.x,
                    y: point_b.y,
                };
                let other_point_2 = Point {
                    x: point_b.x,
                    y: point_a.y,
                };

                if !polygon.point_in_polygon(&other_point_1)
                    || !polygon.point_in_polygon(&other_point_2)
                {
                    continue;
                }
            }

            // Check area will be in polygon
            {
                if polygon.edges_cross(&point_a, &point_b) {
                    continue;
                }
            }

            let x = point_a.x.abs_diff(point_b.x) + 1;
            let y = point_a.y.abs_diff(point_b.y) + 1;

            let area = x * y;
            all_areas.push(area);
        }
    }

    all_areas.sort_unstable();
    all_areas.reverse();
    all_areas[0]
}
