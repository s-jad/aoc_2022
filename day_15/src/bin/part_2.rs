use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn find_coverage(
    sensor: (isize, isize),
    beacon: (isize, isize),
    grid_min: (isize, isize),
    grid_max: (isize, isize),
) -> HashSet<(isize, isize)> {
    let mut coverage = HashSet::new();

    let sb_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

    let mut up = (sensor.0, sensor.1 + sb_distance);
    let mut down = (sensor.0, sensor.1 - sb_distance);
    let mut left = (sensor.0 - sb_distance, sensor.1);
    let mut right = (sensor.0 + sb_distance, sensor.1);

    up = (up.0.min(grid_max.0), up.1.min(grid_max.1));
    down = (down.0.max(grid_min.0), down.1.max(grid_min.1));
    left = (left.0.max(grid_min.0), left.1.max(grid_min.1));
    right = (right.0.min(grid_max.0), right.1.min(grid_max.1));

    for i in left.0..=right.0 {
        for j in down.1..=up.1 {
            if (i - sensor.0).abs() + (j - sensor.1).abs() <= sb_distance {
                coverage.insert((i, j));
            }
        }
    }

    coverage
}

fn process(input: &str) -> isize {
    const GRID_MIN: (isize, isize) = (0, 0);
    const GRID_MAX: (isize, isize) = (4_000_000, 4_000_000);

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

    let mut total_coverage = HashSet::new();
    for i in 0..sb.len() {
        let coverage = find_coverage(sb[i].0, sb[i].1, GRID_MIN, GRID_MAX);
        total_coverage.extend(coverage);
    }
    let mut signal_strength = 0;
    for i in 0..4_000_000 {
        for j in 0..4_000_000 {
            if !total_coverage.contains(&(i, j)) {
                signal_strength = (i * 4_000_000) + j;
            }
        }
    }

    signal_strength
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
