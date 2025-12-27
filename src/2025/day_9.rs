use std::fs::read_to_string;

#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn rectangle_in_polygon(p1_t: &Point, p2_t: &Point, points: &[Point]) -> bool {
    let p1 = Point {
        x: p1_t.x.min(p2_t.x),
        y: p1_t.y.min(p2_t.y),
    };
    let p2 = Point {
        x: p1_t.x.max(p2_t.x),
        y: p1_t.y.max(p2_t.y),
    };

    let len = points.len();

    for i in 0..len {
        let j = (i + len - 1) % len;
        let p_i = points[i].clone();
        let p_j = points[j].clone();

        assert!(p_i.x == p_j.x || p_i.y == p_j.y);

        if p_i.x == p_j.x {
            // Vertical edge
            if p_i.x > p1.x && p_i.x < p2.x {
                let y_min = p_i.y.min(p_j.y);
                let y_max = p_i.y.max(p_j.y);

                if !(p2.y <= y_min || p1.y >= y_max) {
                    return false;
                }
            }
        } else {
            // p_i.y == p_j.y
            // Horizontal edge
            if p_i.y > p1.y && p_i.y < p2.y {
                let x_min = p_i.x.min(p_j.x);
                let x_max = p_i.x.max(p_j.x);

                if !(p2.x <= x_min || p1.x >= x_max) {
                    return false;
                }
            }
        }
    }

    true
}

fn part_2(points: &[Point]) {
    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            if rectangle_in_polygon(p1, p2, points) {
                let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    println!("Part 2: {}", max_area);
}

fn part_1(points: &[Point]) {
    let mut max_area = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            let area = ((p2.x - p1.x).abs() + 1) * ((p2.y - p1.y).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Part 1: {}", max_area);
}

fn main() {
    let contents = read_to_string("assets/2025/day_9.input").unwrap();
    // let contents = read_to_string("assets/2025/test.input").unwrap();

    let points = contents
        .lines()
        .map(|line| {
            let parts = line.split(',').collect::<Vec<&str>>();
            Point {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            }
        })
        .collect::<Vec<Point>>();

    part_1(&points);
    part_2(&points);
}
