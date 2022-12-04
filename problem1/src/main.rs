use std::{fs::File, io::Read};

struct Elf {
    calories: Vec<u32>,
}
fn main() {
    let mut f = File::open("problem1/input.txt").unwrap();
    let mut buff = String::new();
    f.read_to_string(&mut buff).unwrap();

    let lines = buff.lines().collect::<Vec<_>>();
    let elves: Vec<Elf> = lines
        .split(|line| line.is_empty())
        .map(|str_calories| {
            str_calories
                .iter()
                .map(|&str_calories| str_calories.parse().unwrap())
                .collect()
        })
        .map(|calories| Elf { calories })
        .collect();

    let mut cal_per_elf: Vec<u32> = elves
        .iter()
        .map(|elf| elf.calories.iter().sum::<u32>())
        .collect();

    cal_per_elf.sort();

    println!("Most calories held by elf: {}", cal_per_elf.last().unwrap());

    println!(
        "Sum of top three: {}",
        cal_per_elf.iter().rev().take(3).sum::<u32>()
    );
}
