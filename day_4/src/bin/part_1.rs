use core::ops::RangeInclusive;
use itertools::Itertools;

fn process(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            line.split_terminator(&[',', '-'][..])
                .map(|s| s.parse::<usize>().expect("Should be a number"))
                .tuples::<(usize, usize)>()
                .map(|(start, end)| RangeInclusive::new(start, end))
                .collect_tuple::<(RangeInclusive<usize>, RangeInclusive<usize>)>()
                .filter(|(r1, r2)| {
                    r1.contains(&r2.start())
                        || r1.contains(&r2.end())
                        || r2.contains(&r1.start())
                        || r2.contains(&r1.end())
                })
        })
        .count()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
