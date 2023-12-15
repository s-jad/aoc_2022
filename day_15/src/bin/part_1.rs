use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn find_coverage(sensor: (isize, isize), beacon: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut coverage = HashSet::from([sensor]);
    coverage.insert(sensor);

    let steps = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

    let mut current_coverage = HashSet::from([sensor]);

    while !coverage.contains(&beacon) {
        for point in current_coverage.clone().into_iter() {
            for step in steps {
                coverage.insert((point.0 + step.0, point.1 + step.1));
            }
        }
        current_coverage = coverage.clone();
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
        let mut coverage = find_coverage(sb[i].0, sb[i].1);
        coverage.retain(|(_, y)| y == &Y_ROW);
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
