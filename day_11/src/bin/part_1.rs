use itertools::Itertools;
use std::cell::RefCell;

struct Monkey {
    id: usize,
    items: RefCell<Vec<usize>>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: RefCell<usize>,
}

fn process(input: &str) -> usize {
    let paras = input.split_terminator("\n\n").collect_vec();

    let monkeys: Vec<Monkey> = paras
        .iter()
        .map(|&m| {
            let lines = m.lines().collect_vec();
            let id = lines[0]
                .split_terminator(&[' ', ':'][..])
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec()[0];

            let items: Vec<usize> = lines[1]
                .split_terminator(&[' ', ','][..])
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            let op_str = lines[2].split_once("=").unwrap().1;
            let op_parts = op_str.split_whitespace().collect_vec();
            let operator_is_digit = op_parts[2].parse::<usize>().ok();
            let operation: Box<dyn Fn(usize) -> usize> = match (op_parts[1], operator_is_digit) {
                ("+", Some(n)) => Box::new(move |x: usize| x + n),
                ("*", Some(n)) => Box::new(move |x: usize| x * n),
                ("+", None) => Box::new(|x: usize| x + x),
                ("*", None) => Box::new(|x: usize| x * x),
                _ => Box::new(|x| x),
            };

            let test_num = lines[3]
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec()[0];

            let test = Box::new(move |x: usize| x % test_num == 0);

            let true_monkey = lines[4]
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec()[0];

            let false_monkey = lines[5]
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_vec()[0];

            Monkey {
                id,
                items: RefCell::new(items),
                operation,
                test,
                true_monkey,
                false_monkey,
                inspection_count: RefCell::new(0),
            }
        })
        .collect_vec();

    for _ in 1..=20 {
        for j in 0..monkeys.len() {
            let mut items = monkeys[j].items.borrow_mut();
            for item in items.drain(0..) {
                let mut worry_level = (monkeys[j].operation)(item);
                worry_level = (worry_level as f64 / 3f64).floor() as usize;
                let throw_to = (monkeys[j].test)(worry_level);
                *monkeys[j].inspection_count.borrow_mut() += 1;

                if throw_to {
                    let partner = monkeys[j].true_monkey;
                    monkeys[partner].items.borrow_mut().push(worry_level);
                } else {
                    let partner = monkeys[j].false_monkey;
                    monkeys[partner].items.borrow_mut().push(worry_level);
                }
            }
        }
    }

    let mut most_active: Vec<usize> = Vec::new();
    for monkey in monkeys.iter() {
        let ins = *monkey.inspection_count.borrow();
        most_active.push(ins);
    }
    most_active.sort();
    let len = most_active.len();
    let max1 = most_active[len - 1]; // Last element is the maximum
    let max2 = most_active[len - 2]; // S
    max1 * max2
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
