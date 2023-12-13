use itertools::Itertools;

fn process(input: &str) -> isize {
    let ins = input
        .lines()
        .map(|l| {
            let split = l.split_whitespace().collect_vec();
            if split[0] == "noop" {
                ("noop", 0isize)
            } else {
                (split[0], split[1].parse::<isize>().expect("number"))
            }
        })
        .collect_vec();

    let mut corrected_ins = Vec::new();

    for (idx, _) in ins.iter().enumerate() {
        if ins[idx].0 == "addx" {
            corrected_ins.push(0);
            corrected_ins.push(ins[idx].1);
        } else {
            corrected_ins.push(0);
        }
    }

    println!("ins => {:?}", ins);
    let mut total: isize = 1;
    let mut signal_str: Vec<isize> = Vec::new();

    for (idx, instruction) in corrected_ins.into_iter().enumerate() {
        let mut step = idx as isize + 1;
        total += instruction;

        println!("step => {:?}", step);

        if (step - 20) % 40 == 0 {
            signal_str.push(total * step);
            println!("total => {:?}", total);
            println!("signal => {:?}", total * step);
        }
    }

    signal_str.iter().sum()
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
