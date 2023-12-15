use itertools::Itertools;
use std::collections::HashSet;

pub fn process(input: &str) -> usize {
    let rocks: Vec<(usize, usize)> = input
        .lines()
        .flat_map(|l| {
            l.split_terminator(" -> ")
                .tuple_windows()
                .flat_map(|(c1, c2)| {
                    let (x1, y1) = c1
                        .split_terminator(",")
                        .map(|s| s.parse::<usize>().expect("s is a number"))
                        .collect_tuple()
                        .expect("Should be a tuple");

                    let (x2, y2) = c2
                        .split_terminator(",")
                        .map(|s| s.parse::<usize>().expect("s is a number"))
                        .collect_tuple()
                        .expect("Should be a tuple");

                    let max_x = x1.max(x2);
                    let min_x = x1.min(x2);
                    let max_y = y1.max(y2);
                    let min_y = y1.min(y2);
                    let x_range = min_x..=max_x;
                    let y_range = min_y..=max_y;

                    x_range.cartesian_product(y_range).collect_vec()
                })
                .collect::<HashSet<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    let mut floor = 0;

    for i in 0..rocks.len() {
        if rocks[i].1 > floor {
            floor = rocks[i].1;
        }
    }
    floor += 2;

    let mut rock_set = rocks.into_iter().collect::<HashSet<(usize, usize)>>();
    let mut sand_count = 1;
    let mut prev_row: HashSet<(usize, usize)> = HashSet::from([(500, 0)]);

    for _ in 1..floor {
        let mut next_row = HashSet::new();
        for point in prev_row.iter() {
            let below = (point.0, point.1 + 1);
            let diag_l = (point.0 - 1, point.1 + 1);
            let diag_r = (point.0 + 1, point.1 + 1);

            if !rock_set.contains(&below) {
                sand_count += 1;
                next_row.insert(below);
                rock_set.insert(below);
            }

            if !rock_set.contains(&diag_l) {
                sand_count += 1;
                next_row.insert(diag_l);
                rock_set.insert(diag_l);
            }
            if !rock_set.contains(&diag_r) {
                sand_count += 1;
                next_row.insert(diag_r);
                rock_set.insert(diag_r);
            }
        }
        prev_row = next_row;
    }

    sand_count
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
