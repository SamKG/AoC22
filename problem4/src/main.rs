use itertools::Itertools;
use std::{fmt::Display, fs::File, io::Read};

#[derive(Debug, Copy, Clone)]
struct Range {
    left: usize,
    right: usize,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.left, self.right)
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.left <= other.left && self.right >= other.right
    }
    fn intersection(&self, other: &Range) -> Option<Range> {
        let (l, r) = (self.left.max(other.left), self.right.min(other.right));
        match l <= r {
            true => Some(Range { left: l, right: r }),
            false => None,
        }
    }
    fn overlaps(&self, other: &Range) -> bool {
        self.intersection(other).is_some()
    }
}

fn str_to_range(s: &str) -> Range {
    let mut split = s.split('-');
    Range {
        left: split.next().unwrap().parse().unwrap(),
        right: split.next().unwrap().parse().unwrap(),
    }
}
fn main() {
    let mut f = File::open("problem4/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut ranges = buf
        .lines()
        .flat_map(|l| l.split(","))
        .map(str_to_range)
        .collect::<Vec<_>>();

    let count: u64 = ranges
        .chunks_exact(2)
        .map(|range| {
            let (a, b) = (range[0], range[1]);
            a.contains(&b) || b.contains(&a)
        })
        .map(|b| b as u64)
        .sum();

    println!("Pair assignments where fully contained: {}", count);

    let count: u64 = ranges
        .chunks_exact(2)
        .map(|range| {
            let (a, b) = (range[0], range[1]);
            a.overlaps(&b)
        })
        .map(|b| b as u64)
        .sum();

    println!("Assignments where overlaps: {}", count);
}
