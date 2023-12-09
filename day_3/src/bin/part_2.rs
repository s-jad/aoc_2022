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
        .tuples::<(_, _, _)>()
        .map(|(l1, l2, l3)| {
            for c in l1.chars() {
                if l2.contains(c) && l3.contains(c) {
                    return map_ascii_val(c) as usize;
                }
            }
            return 0;
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
