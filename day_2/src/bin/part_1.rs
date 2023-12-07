use itertools::Itertools;

fn process(input: &str) -> u64 {
    let combos: [(&str, [(&str, u8); 3]); 3] = [
        ("A", [("X", 4), ("Y", 8), ("Z", 3)]),
        ("B", [("X", 1), ("Y", 5), ("Z", 9)]),
        ("C", [("X", 7), ("Y", 2), ("Z", 6)]),
    ];

    input
        .lines()
        .flat_map(|line| {
            line.split_whitespace().tuples().map(|(o_move, my_move)| {
                let combo = combos.iter().find(|&&(c, _)| c == o_move).unwrap();
                match combo {
                    (_, moves) => moves
                        .iter()
                        .find(|&&(c, _)| c == my_move)
                        .map(|&(_, value)| value as u64)
                        .expect("should be a value"),
                }
            })
        })
        .sum()
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
