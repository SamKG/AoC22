use std::{fs::File, io::Read, path::Path};

fn solve_part1(input: &String) -> i64 {
    todo!();
}

fn solve_part2(input: &String) -> i64 {
    todo!();
}

fn main() {
    let root_dir = Path::new(file!()).ancestors().nth(2).unwrap();
    let f_path = root_dir.join("input.txt");

    let mut f = File::open(f_path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let ans_1 = solve_part1(&buf);
    println!("Answer 1: {}", ans_1);

    let ans_2 = solve_part2(&buf);
    println!("Answer 2: {}", ans_2);
}