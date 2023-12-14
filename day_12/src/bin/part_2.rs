use itertools::Itertools;
use pathfinding::prelude::astar;

fn heuristic(pos: (usize, usize), goal: &(usize, usize)) -> usize {
    let (x, y) = pos;
    let (goal_x, goal_y) = goal;

    ((*goal_x as i32 - x as i32).abs() + (*goal_y as i32 - y as i32).abs()) as usize
}

fn successors(pos: (usize, usize), grid: &Vec<Vec<u8>>) -> Vec<((usize, usize), usize)> {
    let (x, y) = pos;
    let mut successors = Vec::new();

    let offsets: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dx, dy) in offsets.iter() {
        let new_x = (x as i32 + dx) as i32;
        let new_y = (y as i32 + dy) as i32;

        if new_x >= 0
            && (new_x as usize) < grid[0].len()
            && new_y >= 0
            && (new_y as usize) < grid.len()
        {
            if grid[y][x] as i32 - grid[new_y as usize][new_x as usize] as i32 >= -1 {
                successors.push(((new_x as usize, new_y as usize), 1));
            }
        }
    }
    successors
}

fn process(input: &str) -> usize {
    let mut starting_positions: Vec<(usize, usize)> = Vec::new();
    let mut goal: (usize, usize) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => starting_positions.push((x, y)),
                'E' => goal = (x, y),
                'a' => starting_positions.push((x, y)),
                _ => {}
            }
        }
    }

    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'E' {
                        26
                    } else if c == 'S' {
                        1
                    } else {
                        c as u8 - 96
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut paths: Vec<usize> = starting_positions
        .into_iter()
        .filter_map(|start| {
            let path = astar(
                &start,
                |&p| successors(p, &grid),
                |&p| heuristic(p, &goal),
                |&p| p == goal,
            );

            match path {
                Some(path) => Some(path.1),
                None => None,
            }
        })
        .collect_vec();

    paths.sort();

    paths[0]
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
        assert_eq!(result,);
    }
}
