use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut f = File::open("problem5/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let n_cols = buf.lines().next().unwrap().len() / 4 + 1;
    let mut boxes: Vec<Vec<String>> = vec![vec![]; n_cols];

    let n_rows = buf
        .lines()
        .enumerate()
        .find(|(i, l)| l.is_empty())
        .unwrap()
        .0;

    let box_lines = buf.lines().take(n_rows - 1);
    let box_pattern = Regex::new(r"(\w+)").unwrap();
    box_lines
        .flat_map(|l| box_pattern.captures_iter(l))
        .for_each(|cap| {
            let val = cap.get(0).unwrap();
            boxes[(val.start() / 4)].insert(0, val.as_str().to_string());
        });

    let move_pattern =
        Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    println!("{boxes:?}");

    let mut boxes_problem1 = boxes.clone();
    buf.lines()
        .filter(|l| l.starts_with("move"))
        .flat_map(|l| move_pattern.captures_iter(l))
        .for_each(|c| {
            let count = c["count"].parse().unwrap();
            let from: usize = (c["from"].parse::<i32>().unwrap() - 1) as usize;
            let to: usize = (c["to"].parse::<i32>().unwrap() - 1) as usize;
            for _ in 0..count {
                match boxes_problem1[from].pop() {
                    Some(b) => boxes_problem1[to].push(b),
                    None => panic!(),
                }
            }
        });

    print!("Answer part 1: ");
    boxes_problem1
        .iter()
        .filter_map(|b| b.get(b.len() - 1).and_then(|b| Some(b)))
        .for_each(|s| {
            print!("{s}");
        });
    print!("\n");

    let mut boxes_problem2 = boxes.clone();
    buf.lines()
        .filter(|l| l.starts_with("move"))
        .flat_map(|l| move_pattern.captures_iter(l))
        .for_each(|c| {
            let count = c["count"].parse().unwrap();
            let from: usize = (c["from"].parse::<i32>().unwrap() - 1) as usize;
            let to: usize = (c["to"].parse::<i32>().unwrap() - 1) as usize;
            let mut tmp = vec![];
            for _ in 0..count {
                match boxes_problem2[from].pop() {
                    Some(b) => tmp.insert(0, b),
                    None => panic!(),
                }
            }
            boxes_problem2[to].append(&mut tmp);
        });

    print!("Answer part 2: ");
    boxes_problem2
        .iter()
        .filter_map(|b| b.get(b.len() - 1).and_then(|b| Some(b)))
        .for_each(|s| {
            print!("{s}");
        });
    print!("\n");
}
