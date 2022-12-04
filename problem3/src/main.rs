use std::{collections::HashSet, fs::File, io::Read};

fn to_priority(shared: char) -> u8 {
    match shared {
        'a'..='z' => (shared as u8) - ('a' as u8) + 1,
        'A'..='Z' => (shared as u8) - ('A' as u8) + 27,
        _ => panic!("Invalid character"),
    }
}

fn main() {
    let mut f = File::open("problem3/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut items: Vec<(HashSet<char>, HashSet<char>)> = Vec::new();

    for line in buf.lines().map(str::to_string) {
        let (part1, part2) = (&line[..line.len() / 2], &line[line.len() / 2..]);
        let items_1 = part1.chars().collect::<HashSet<_>>();
        let items_2 = part2.chars().collect::<HashSet<_>>();

        items.push((items_1, items_2));
    }

    let total_priority = items
        .iter()
        .map(|(items_1, items_2)| {
            let shared = items_1.intersection(&items_2).next().unwrap();
            to_priority(*shared) as u64
        })
        .sum::<u64>();

    println!("Total priority: {}", total_priority);

    let elf_group_badges: Vec<u64> = items
        .chunks_exact(3)
        .map(|s| {
            s.iter()
                .fold(HashSet::new(), |acc, (x, y)| {
                    let union: HashSet<char> = x.union(&y).cloned().collect();
                    match acc.is_empty() {
                        true => union,
                        false => acc.intersection(&union).cloned().collect(),
                    }
                })
                .iter()
                .cloned()
                .next()
                .unwrap()
        })
        .map(to_priority)
        .map(|x| x as u64)
        .collect();

    println!(
        "Elf group badges: {:?}",
        elf_group_badges.iter().sum::<u64>()
    );
}
