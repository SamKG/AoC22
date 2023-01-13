use std::{collections::BinaryHeap, fs::File, io::Read};

use evalexpr::{build_operator_tree, Context, ContextWithMutableVariables, HashMapContext, Node};
use regex::Regex;

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    divisor: i64,
    op: Node,
    true_target: i64,
    false_target: i64,
    inspect_count: i64,
}

impl From<String> for Monkey {
    fn from(value: String) -> Self {
        let (mut items, mut divisor, mut op_tree, mut true_target, mut false_target) =
            (None, None, None, None, None);

        for mut tokens in value.lines().map(str::split_whitespace) {
            match tokens.next() {
                Some("Monkey") => {}
                Some("Starting") => {
                    let _ = tokens.next();
                    let re = Regex::new(r"(\d+).*").unwrap();
                    items = Some(
                        tokens
                            .map(|s| &re.captures(s).unwrap().get(1).unwrap().as_str()[..])
                            .map(str::parse)
                            .map(Result::unwrap)
                            .collect(),
                    );
                }
                Some("Operation:") => {
                    let expr = tokens.collect::<Vec<&str>>().join(" ");
                    op_tree = Some(build_operator_tree(&expr).unwrap());
                }
                Some("Test:") => {
                    divisor = tokens.last().map(str::parse::<i64>).map(Result::unwrap);
                }
                Some("If") => match tokens.next() {
                    Some("true:") => {
                        true_target = tokens.last().map(str::parse::<i64>).map(Result::unwrap);
                    }
                    Some("false:") => {
                        false_target = tokens.last().map(str::parse::<i64>).map(Result::unwrap);
                    }
                    _ => panic!("Invalid input!"),
                },
                None => panic!("Invalid input!"),
                Some(&_) => panic!("Invalid input!"),
            }
        }

        Monkey {
            items: items.unwrap(),
            divisor: divisor.unwrap(),
            op: op_tree.unwrap(),
            true_target: true_target.unwrap(),
            false_target: false_target.unwrap(),
            inspect_count: 0,
        }
    }
}

fn solve_part1(input: &String) -> i64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|c| c.join("\n"))
        .map(Monkey::from)
        .collect();


    for i in 1..=20 {
        for idx in 0..monkeys.len() {
            let mut monkey = monkeys[idx].clone();
            for item in monkey.items.drain(..) {
                let mut context = HashMapContext::new();
                context
                    .set_value("old".to_string(), evalexpr::Value::Int(item.into()))
                    .unwrap();

                monkey.op.eval_with_context_mut(&mut context).unwrap();
                let result = (context.get_value("new").unwrap().as_int().unwrap() / 3);

                if result % monkey.divisor as i64 == 0 {
                    monkeys[monkey.true_target as usize]
                        .items
                        .push(result as i64);
                } else {
                    monkeys[monkey.false_target as usize]
                        .items
                        .push(result as i64);
                };
                monkey.inspect_count += 1;
            }
            monkeys[idx] = monkey.clone();
        }
    }

    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected {} items", idx, monkey.inspect_count);
    }

    let mut heap = BinaryHeap::from(
        monkeys
            .iter()
            .map(|m| m.inspect_count)
            .collect::<Vec<i64>>(),
    );

    let (a, b) = (heap.pop().unwrap(), heap.pop().unwrap());

    println!("Top 2: {:?}", [a, b]);

    a * b
}

fn solve_part2(input: &String) -> i64 {
    let mut monkeys: Vec<Monkey> = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|c| c.join("\n"))
        .map(Monkey::from)
        .collect();

    let divisor:i64 = monkeys.iter().map(|m| m.divisor).product();
    
    for i in 1..=10000 {
        for idx in 0..monkeys.len() {
            let mut monkey = monkeys[idx].clone();
            for item in monkey.items.drain(..) {
                let mut context = HashMapContext::new();
                context
                    .set_value("old".to_string(), evalexpr::Value::Int(item.into()))
                    .unwrap();

                monkey.op.eval_with_context_mut(&mut context).unwrap();
                let result = (context.get_value("new").unwrap().as_int().unwrap() % divisor);

                if result % monkey.divisor as i64 == 0 {
                    monkeys[monkey.true_target as usize]
                        .items
                        .push(result as i64);
                } else {
                    monkeys[monkey.false_target as usize]
                        .items
                        .push(result as i64);
                };
                monkey.inspect_count += 1;
            }
            monkeys[idx] = monkey.clone();
        }
    }

    for (idx, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected {} items", idx, monkey.inspect_count);
    }

    let mut heap = BinaryHeap::from(
        monkeys
            .iter()
            .map(|m| m.inspect_count)
            .collect::<Vec<i64>>(),
    );

    let (a, b) = (heap.pop().unwrap(), heap.pop().unwrap());

    println!("Top 2: {:?}", [a, b]);

    a * b
}

fn main() {
    let mut f = File::open("problem11/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let ans_1 = solve_part1(&buf);
    println!("Answer 1: {}", ans_1);

    let ans_2 = solve_part2(&buf);
    println!("Answer 2: {}", ans_2);
}

// write test solve_part1 wth input.txt

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        let mut f = File::open("test.txt").unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        let ans = solve_part1(&buf);

        let expected = 10605;
        assert_eq!(ans, expected, "Expected {expected}, got {ans}");
    }
}
