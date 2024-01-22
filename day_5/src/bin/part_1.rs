use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn process(input: &str) -> String {
    let (g_str, i_str) = input.split_once("\n\n").expect("Should split");
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();

    g_str
        .lines()
        .flat_map(|l| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| c.is_alphabetic())
                .map(|(x, c)| {
                    let ax = (x as i32 / 4) as usize;
                    (c, ax)
                })
                .collect_vec()
        })
        .for_each(|(c, x)| {
            stacks
                .entry(x + 1)
                .or_insert_with(VecDeque::new)
                .push_back(c)
        });

    let instructions = i_str
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
        })
        .collect_vec();

    for ins in instructions {
        let mut temp = vec![];

        for _ in 0..ins.0 {
            let start = stacks.get_mut(&ins.1).unwrap();
            let moved = start.pop_front().unwrap();
            temp.push(moved);
        }

        for m in temp {
            let end = stacks.get_mut(&ins.2).unwrap();
            end.push_front(m);
        }
    }

    let mut final_str = String::new();

    for i in 1..=stacks.len() {
        let stack = stacks.get_mut(&i).unwrap();
        let s = stack.pop_front().unwrap();
        final_str.push(s);
    }

    final_str
}

fn main() {
    let input = include_str!("../../input.txt");
    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();
    dbg!(output, time);
}
