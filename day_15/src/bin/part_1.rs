use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn find_coverage(
    sensor: (isize, isize),
    beacon: (isize, isize),
    y_row: isize,
) -> HashSet<(isize, isize)> {
    let mut coverage = HashSet::new();

    let sb_distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
    let sy_distance = (sensor.1 - y_row).abs();
    let sy_coverage = sb_distance - sy_distance;

    if sy_coverage > 0 {
        let min_x = sensor.0 - sy_coverage;
        let max_x = sensor.0 + sy_coverage;

        for i in min_x..=max_x {
            coverage.insert((i, y_row));
        }
    }
    coverage
}

fn process(input: &str) -> usize {
    const Y_ROW: isize = 2_000_000;

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
        let coverage = find_coverage(sb[i].0, sb[i].1, Y_ROW);
        total_coverage.extend(coverage);
    }

    let y_beacons = sb
        .iter()
        .filter(|((_, _), (_, y))| y == &Y_ROW)
        .map(|(_, b)| b)
        .unique()
        .count();

    total_coverage.len() - y_beacons
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
