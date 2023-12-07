fn process(input: &str) -> u64 {
    input
        .split_terminator("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|food| food.parse::<u64>().expect("Should be a number"))
                .sum()
        })
        .max()
        .expect("There will be a maximum")
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = process(input);

    dbg!(output);
}
