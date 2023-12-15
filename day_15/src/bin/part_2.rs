use geo::{line_intersection::line_intersection, Line, LineIntersection};
use itertools::Itertools;
use std::collections::HashMap;
use std::time::Instant;

fn most_frequent<T: std::hash::Hash + Eq>(vec: Vec<T>) -> Option<T> {
    let mut counts = HashMap::new();
    for item in vec {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(item, _)| item)
}

fn find_boundary(sensor: (isize, isize), beacon: (isize, isize)) -> Vec<Line<f64>> {
    let sb_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

    let left = (sensor.0 - sb_distance) - 1;
    let right = sensor.0 + sb_distance + 1;
    let up = (sensor.1 - sb_distance) - 1;
    let down = sensor.1 + sb_distance + 1;

    let tl = Line::new((sensor.0 as f64, up as f64), (left as f64, sensor.1 as f64));
    let tr = Line::new(
        (sensor.0 as f64, up as f64),
        (right as f64, sensor.1 as f64),
    );
    let bl = Line::new(
        (sensor.0 as f64, down as f64),
        (left as f64, sensor.1 as f64),
    );
    let br = Line::new(
        (sensor.0 as f64, down as f64),
        (right as f64, sensor.1 as f64),
    );

    vec![tl, tr, bl, br]
}

fn diagonal_walk(start: (isize, isize), end: (isize, isize)) -> Vec<(isize, isize)> {
    let mut points = Vec::new();
    let mut x1 = start.0;
    let mut y1 = start.1;
    let x2 = end.0;
    let y2 = end.1;

    let sx = if x1 - x2 > 0 { -1 } else { 1 };
    let sy = if y1 - y2 > 0 { -1 } else { 1 };

    while x1 != x2 || y1 != y2 {
        if x1 >= 0 && x1 <= 4_000_000 && y1 >= 0 && y1 <= 4_000_000 {
            points.push((x1, y1));
        }
        x1 += sx;
        y1 += sy;
    }

    points.push((x1, y1));
    points
}

fn process(input: &str) -> isize {
    let sb = input
        .lines()
        .flat_map(|l| {
            l.split_terminator(&[' ', '=', ':', ','][..])
                .filter_map(|s| s.parse::<isize>().ok())
                .collect_vec()
                .chunks_exact(4)
                .map(|c| ((c[0], c[1]), (c[2], c[3])))
                .collect_vec()
        })
        .collect_vec();

    let mut bounding_lines = Vec::new();
    for (s, b) in sb.into_iter() {
        bounding_lines.extend(find_boundary(s, b));
    }

    let mut intersection_lines = Vec::new();

    for i in 0..bounding_lines.len() {
        for j in i..bounding_lines.len() {
            match line_intersection(bounding_lines[i], bounding_lines[j]) {
                Some(intersection) => match intersection {
                    LineIntersection::SinglePoint {
                        intersection: _point,
                        ..
                    } => {}
                    LineIntersection::Collinear {
                        intersection: line, ..
                    } => {
                        intersection_lines.push(line);
                    }
                },
                None => {}
            }
        }
    }

    let mut points_vec = Vec::new();

    for line in intersection_lines {
        let (start, end) = line.points();
        points_vec.extend(diagonal_walk(
            (start.x() as isize, start.y() as isize),
            (end.x() as isize, end.y() as isize),
        ));
    }

    let unobserved = most_frequent(points_vec.clone()).unwrap();

    (unobserved.0 * 4_000_000) + unobserved.1
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
