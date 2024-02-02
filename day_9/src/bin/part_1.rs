use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
struct Point(isize, isize);

impl Point {
    fn check_neighbours(&self, other: &Point) -> bool {
        let diff = *self - *other;
        if diff == Point(1, 0)
            || diff == Point(0, 1)
            || diff == Point(-1, 0)
            || diff == Point(0, -1)
            || diff == Point(1, 1)
            || diff == Point(1, -1)
            || diff == Point(-1, 1)
            || diff == Point(-1, -1)
        {
            true
        } else {
            false
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = Self(self.0 - other.0, self.1 - other.1);
    }
}

fn process(input: &str) -> usize {
    let instructions = input
        .lines()
        .map(|line| {
            let (dir, count) = line.split_whitespace().collect_tuple().unwrap();

            (dir, count.parse::<isize>().expect("Should be number"))
        })
        .collect_vec();

    let dirs: HashMap<&str, Point> = HashMap::from([
        ("R", Point(1, 0)),
        ("L", Point(-1, 0)),
        ("U", Point(0, 1)),
        ("D", Point(0, -1)),
    ]);

    let mut h = Point(0, 0);
    let mut t = Point(0, 0);
    let mut visited: HashSet<Point> = HashSet::new();

    for i in instructions {
        let dir = dirs.get_key_value(i.0).unwrap();
        for _ in 0..i.1 {
            let new_h = Point(h.0 + dir.1 .0, h.1 + dir.1 .1);
            if !(new_h.check_neighbours(&t) || new_h == t) {
                let mut new_t = Point(0, 0);

                let d_ht = new_h - t;

                let mut diag = Point(0, 0);
                if d_ht.0.abs() > 0 && d_ht.1.abs() > 0 {
                    let sx = if d_ht.0 > 0 { 1 } else { -1 };
                    let sy = if d_ht.1 > 0 { 1 } else { -1 };
                    diag = Point(sx, sy);
                }

                match *dir.0 {
                    "U" => new_t = Point(new_h.0, new_h.1 - 1),
                    "D" => new_t = Point(new_h.0, new_h.1 + 1),
                    "R" => new_t = Point(new_h.0 - 1, new_h.1),
                    "L" => new_t = Point(new_h.0 + 1, new_h.1),
                    _ => {}
                }

                t = t + diag;
                visited.insert(t);
                t = new_t;
            }
            h = new_h;
            visited.insert(t);
        }
    }

    visited.len()
}

fn main() {
    let input = include_str!("../../input.txt");
    let time = Instant::now();
    let output = process(input);
    let t = time.elapsed();
    dbg!(output, t);
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

