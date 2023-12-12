use itertools::Itertools;

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

    let mut tree_scores: Vec<usize> = Vec::new();

    for tree in grid.iter() {
        let mut tree_score = 0;

        let visible_up = grid
            .iter()
            .filter(|((x, y), _)| x == &tree.0 .0 && y < &tree.0 .1)
            .rev()
            .take_while_inclusive(|other| other.1 < tree.1)
            .count();

        let visible_down = grid
            .iter()
            .filter(|((x, y), _)| x == &tree.0 .0 && y > &tree.0 .1)
            .take_while_inclusive(|other| other.1 < tree.1)
            .count();

        let visible_left = grid
            .iter()
            .filter(|((x, y), _)| y == &tree.0 .1 && x < &tree.0 .0)
            .rev()
            .take_while_inclusive(|other| other.1 < tree.1)
            .count();

        let visible_right = grid
            .iter()
            .filter(|((x, y), _)| y == &tree.0 .1 && x > &tree.0 .0)
            .take_while_inclusive(|other| other.1 < tree.1)
            .count();

        tree_score = visible_up * visible_down * visible_left * visible_right;

        tree_scores.push(tree_score);
    }

    *tree_scores.iter().max().unwrap()
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
