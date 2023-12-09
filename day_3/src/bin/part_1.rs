use itertools::Itertools;

fn map_ascii_val(c: char) -> u8 {
    if c.is_lowercase() {
        c as u8 - 96
    } else {
        c as u8 - 38
    }
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let length = line.len();
            line.split_at(length / 2)
        })
        .map(|(s1, s2)| {
            if let Some(num) = s1.chars().find(|c| s2.contains(*c)) {
                map_ascii_val(num) as usize
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
