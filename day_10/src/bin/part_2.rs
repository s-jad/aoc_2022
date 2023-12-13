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

    let mut sprite: [isize; 3] = [0, 1, 2];
    let mut crt_row = String::new();

    for (idx, instruction) in corrected_ins.into_iter().enumerate() {
        let crt_pos = idx as isize % 40;
        if sprite.contains(&crt_pos) {
            crt_row.push('#');
        } else {
            crt_row.push('.');
        }

        sprite[0] += instruction;
        sprite[1] += instruction;
        sprite[2] += instruction;

        let cpu_cycle = idx + 1;

        if idx > 0 && cpu_cycle % 40 == 0 {
            println!("{:?}", crt_row);
            crt_row = "".to_string();
        }
    }
    1
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
