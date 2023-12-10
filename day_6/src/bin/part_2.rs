use itertools::Itertools;

fn process(input: &str) -> usize {
    let c_vec = input.chars().enumerate().collect_vec();

    let unique = c_vec
        .windows(14)
        .find(|&w| w.iter().map(|(_, c)| c).unique().count() == 14)
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
        let output5 = process(inputs[4]);

        assert_eq!(19, output1);
        assert_eq!(23, output2);
        assert_eq!(23, output3);
        assert_eq!(29, output4);
        assert_eq!(26, output5);
    }
}
