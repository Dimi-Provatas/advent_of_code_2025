#[derive(Clone, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
pub fn parse_points(lines: Vec<String>) -> Vec<Point> {
    let mut points = Vec::new();

    for line in lines {
        let coords = line
            .splitn(2, ",")
            .map(|c| -> usize { c.parse().unwrap() })
            .collect::<Vec<usize>>();

        let x = coords[0];
        let y = coords[1];

        points.push(Point { x, y })
    }

    points
}

pub struct Polygon {
    pub lines: Vec<(Point, Point)>,
}
impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        let mut lines = Vec::new();

        for i in 0..points.len() {
            let start_point = points[i].clone();
            let end_point = points[(i + 1) % (points.len())].clone();

            lines.push((start_point, end_point));
        }

        Self { lines }
    }

    pub fn point_in_polygon(&self, point: &Point) -> bool {
        let mut crossings = 0usize;
        for (start_point, end_point) in &self.lines {
            if (start_point.x == end_point.x
                && point.x == start_point.x
                && between(point.y, start_point.y, end_point.y))
                || (start_point.y == end_point.y
                    && point.y == start_point.y
                    && between(point.x, start_point.x, end_point.x))
            {
                return true;
            }

            if start_point.x != end_point.x {
                continue;
            }

            let vertex_x = start_point.x;
            let vertex_y_min = start_point.y.min(end_point.y);
            let vertex_y_max = start_point.y.max(end_point.y);

            if point.y < vertex_y_min || point.y >= vertex_y_max {
                continue;
            }

            if vertex_x > point.x {
                crossings += 1;
            }
        }

        crossings % 2 == 1
    }

    pub fn edges_cross(&self, point_a: &Point, point_b: &Point) -> bool {
        let rect = rect_edges(point_a, point_b);

        for poly_line in &self.lines {
            if poly_line.0 == poly_line.1 {
                continue;
            }
            let seg_is_vert = poly_line.0.x == poly_line.1.x;

            for rect_line in &rect {
                let rect_line_is_vert = rect_line.0.x == rect_line.1.x;

                if seg_is_vert != rect_line_is_vert {
                    if seg_is_vert {
                        if strict_cross(poly_line, rect_line) {
                            return true;
                        }
                    } else if strict_cross(rect_line, poly_line) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn between(p: usize, a: usize, b: usize) -> bool {
    (p >= a && p <= b) || (p <= a && p >= b)
}

fn rect_edges(a: &Point, b: &Point) -> [(Point, Point); 4] {
    let (xmin, xmax) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (ymin, ymax) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };

    let p1 = Point { x: xmin, y: ymin };
    let p2 = Point { x: xmax, y: ymin };
    let p3 = Point { x: xmax, y: ymax };
    let p4 = Point { x: xmin, y: ymax };

    [
        (p1.clone(), p2.clone()),
        (p2.clone(), p3.clone()),
        (p3.clone(), p4.clone()),
        (p4.clone(), p1.clone()),
    ]
}

fn strict_cross(v: &(Point, Point), h: &(Point, Point)) -> bool {
    let vx = v.0.x as isize;
    let (vy0, vy1) = ((v.0.y.min(v.1.y) as isize), (v.0.y.max(v.1.y) as isize));

    let hy = h.0.y as isize;
    let (hx0, hx1) = ((h.0.x.min(h.1.x) as isize), (h.0.x.max(h.1.x) as isize));

    (hy > vy0 && hy < vy1) && (vx > hx0 && vx < hx1)
}
