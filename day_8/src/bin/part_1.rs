use itertools::Itertools;
use pathfinding::prelude::Grid;

fn process(input: &str) -> usize {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(move |(x, c)| ((x, y), c.to_digit(10).expect("all numbers")))
                .collect_vec()
        })
        .collect_vec();
    let ((grid_x_max, grid_y_max), _) = grid.last().unwrap();

    println!("grid_x_max => {:?}", grid_x_max);
    println!("grid_y_max => {:?}", grid_y_max);

    println!("grid => {:?}", grid);

    let mut visible_count = 0;

    for tree in grid.iter() {
        if tree.0 .1 == 0 || tree.0 .0 == 0 || tree.0 .1 == *grid_y_max || tree.0 .0 == *grid_x_max
        {
            visible_count += 1;
        } else {
            let invisible_left = grid
                .iter()
                .filter(|((x, y), _)| x == &tree.0 .0 && y < &tree.0 .1)
                .any(|((_, _), n)| n >= &tree.1);

            let invisible_right = grid
                .iter()
                .filter(|((x, y), _)| x == &tree.0 .0 && y > &tree.0 .1)
                .any(|((_, _), n)| n >= &tree.1);

            let invisible_up = grid
                .iter()
                .filter(|((x, y), _)| y == &tree.0 .1 && x < &tree.0 .0)
                .any(|((_, _), n)| n >= &tree.1);

            let invisible_down = grid
                .iter()
                .filter(|((x, y), _)| y == &tree.0 .1 && x > &tree.0 .0)
                .any(|((_, _), n)| n >= &tree.1);

            println!(
                "invisible => {:?}",
                (
                    invisible_left,
                    invisible_right,
                    invisible_up,
                    invisible_down
                )
            );

            if !invisible_up || !invisible_down || !invisible_left || !invisible_right {
                visible_count += 1;
            }
        }
    }
    println!("visible_count => {:?}", visible_count);
    visible_count
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(21, output);
    }
}
