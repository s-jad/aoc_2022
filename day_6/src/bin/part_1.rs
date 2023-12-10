use itertools::Itertools;

fn process(input: &str) -> usize {
    let c_vec = input.chars().enumerate().collect_vec();

    let unique = c_vec
        .windows(4)
        .find(|&w| w.iter().map(|(_, c)| c).unique().count() == 4)
        .unwrap()
        .iter()
        .last()
        .unwrap();

    unique.0 + 1
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
        let inputs = include_str!("../../test.txt").lines().collect_vec();

        let output1 = process(inputs[0]);
        let output2 = process(inputs[1]);
        let output3 = process(inputs[2]);
        let output4 = process(inputs[3]);

        assert_eq!(5, output1);
        assert_eq!(6, output2);
        assert_eq!(10, output3);
        assert_eq!(11, output4);
    }
}
