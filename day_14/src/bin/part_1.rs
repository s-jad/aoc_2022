use itertools::Itertools;
use std::collections::HashSet;

fn process(input: &str) -> u64 {
    let mut rocks: Vec<(usize, usize)> = input
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

    let mut border_right = 0;

    let mut border_left = 10_000;

    for i in 0..rocks.len() {
        if rocks[i].0 > border_right {
            border_right = rocks[i].0;
        }

        if rocks[i].0 < border_left {
            border_left = rocks[i].0;
        }
    }
    println!("rocks => {:?}", rocks);

    let mut overflowing = false;
    let mut sand_count = 0;

    while !overflowing {
        let mut moving = true;
        let mut sand_pos: (usize, usize) = (500, 0);

        while moving {
            if &sand_pos.0 > &border_right || &sand_pos.0 < &border_left {
                overflowing = true;
                moving = false;
            }

            // Check below
            if rocks.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                // Check diag left
                if rocks.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                    // Check diag right
                    if rocks.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                        rocks.push(sand_pos);
                        sand_count += 1;
                        moving = false;
                    } else {
                        sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                    }
                } else {
                    sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                }
            } else {
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
            }
        }
    }

    sand_count
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
