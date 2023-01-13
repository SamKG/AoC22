use std::{fs::File, io::Read, path::Path};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Number(i64),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(p1), Packet::List(p2)) => {
                let mut self_iter = p1.iter();
                let mut other_iter = p2.iter();
                loop {
                    let self_next = self_iter.next();
                    let other_next = other_iter.next();
                    match (self_next, other_next) {
                        (Some(self_next), Some(other_next)) => {
                            if self_next < other_next {
                                return std::cmp::Ordering::Less;
                            } else if self_next > other_next {
                                return std::cmp::Ordering::Greater;
                            }
                        }
                        (Some(_), None) => return std::cmp::Ordering::Greater,
                        (None, Some(_)) => return std::cmp::Ordering::Less,
                        (None, None) => return std::cmp::Ordering::Equal,
                    }
                }
            }
            (Packet::List(_), Packet::Number(_)) => {
                Packet::cmp(self, &Packet::List(vec![other.clone()]))
            }
            (Packet::Number(_), Packet::List(_)) => {
                Packet::cmp(&Packet::List(vec![self.clone()]), &other)
            }
            (Packet::Number(p1), Packet::Number(p2)) => i64::cmp(p1, p2),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&String> for Packet {
    fn from(text: &String) -> Self {
        let re = regex::Regex::new(r"\s*((?P<number>\d+)|(?P<lbracket>[\[])|(?P<rbracket>[\]])),?")
            .unwrap();
        let mut stack = Vec::new();
        stack.push(Vec::new());
        for cap in re.captures_iter(text) {
            if let Some(number) = cap.name("number") {
                let number = number.as_str().parse::<i64>().unwrap();
                let mut top = stack.pop().unwrap();
                top.push(Packet::Number(number));
                stack.push(top);
            } else if let Some(_) = cap.name("lbracket") {
                stack.push(Vec::new());
            } else if let Some(_) = cap.name("rbracket") {
                let last = stack.pop().unwrap();
                let mut top = stack.pop().unwrap();
                top.push(Packet::List(last));
                stack.push(top);
            }
        }
        return Packet::List(stack.pop().unwrap());
    }
}

fn solve_part1(input: &String) -> i64 {
    let packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(&l.to_string()))
        .collect::<Vec<_>>();

    let mut correct_packets = Vec::new();
    for (idx, chunk) in packets.chunks_exact(2).enumerate() {
        let p1 = &chunk[0];
        let p2 = &chunk[1];
        if p1 < p2 {
            correct_packets.push(idx + 1);
        }
    }
    correct_packets.iter().sum::<usize>() as i64
}

fn solve_part2(input: &String) -> i64 {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(&l.to_string()))
        .collect::<Vec<_>>();

    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort();

    let idx1 = packets.iter().position(|p| p.clone() == divider_1).unwrap();
    let idx2 = packets.iter().position(|p| p.clone() == divider_2).unwrap();

    (idx1 + 1) as i64 * (idx2 + 1) as i64
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
