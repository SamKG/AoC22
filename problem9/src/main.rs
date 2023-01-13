use std::{collections::HashSet, fs::File, io::Read};

struct Bridge {
    seen: HashSet<(i32, i32)>,
    rope: Vec<(i32, i32)>,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Bridge {
    fn new(knots: usize) -> Bridge {
        Bridge {
            seen: HashSet::from([(0, 0)]),
            rope: vec![(0, 0); knots],
        }
    }

    fn step(&mut self, dir: Direction) {
        let mut head = self.rope[0];
        match dir {
            Direction::North => head.1 += 1,
            Direction::South => head.1 -= 1,
            Direction::East => head.0 += 1,
            Direction::West => head.0 -= 1,
        }
        self.rope[0] = head;

        // update rest of rope
        for i in 1..self.rope.len() {
            let (mut curr, prev) = (self.rope[i], self.rope[i - 1]);
            let (dx, dy) = (prev.0 - curr.0, prev.1 - curr.1);

            match dx.pow(2) + dy.pow(2) >= 4 {
                true => {
                    curr.0 += ((dx as f32) / 2.0).round() as i32;
                    curr.1 += ((dy as f32) / 2.0).round() as i32;
                }
                false => {}
            }

            self.rope[i] = curr;
        }

        self.seen.insert(*self.rope.last().unwrap());
    }
}
fn solve_part1(input: &String) -> i32 {
    let mut bridge = Bridge::new(2);
    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let dir = match tokens[0] {
            "U" => Direction::North,
            "D" => Direction::South,
            "R" => Direction::East,
            "L" => Direction::West,
            _ => panic!("Invalid input!"),
        };

        let x = tokens[1].parse::<i32>().unwrap();
        for _ in 0..x {
            bridge.step(dir);
        }
    }

    return bridge.seen.len() as i32;
}
fn solve_part2(input: &String) -> i32 {
    let mut bridge = Bridge::new(10);
    for line in input.lines() {
        let tokens = line.split_whitespace().collect::<Vec<&str>>();
        let dir = match tokens[0] {
            "U" => Direction::North,
            "D" => Direction::South,
            "R" => Direction::East,
            "L" => Direction::West,
            _ => panic!("Invalid input!"),
        };

        let x = tokens[1].parse::<i32>().unwrap();
        for _ in 0..x {
            bridge.step(dir);
        }
    }

    return bridge.seen.len() as i32;
}

fn main() {
    let mut f = File::open("problem9/input.txt").unwrap();
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

        assert_eq!(ans, 13, "Expected 13, got {}", ans);
    }
}
