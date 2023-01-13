use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("problem10/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut current_cycle = 1;
    let mut register: i32 = 1;
    let mut ans_1: i32 = 0;
    let mut ans_2 = String::new();

    let mut pending: Option<i32> = None;

    let mut commands = buf.lines().peekable();
    while commands.peek().is_some() {
        if vec![20, 60, 100, 140, 180, 220].contains(&current_cycle) {
            ans_1 += register * current_cycle;
        }

        // handle CRT logic
        let (crt_x, crt_y) = ((current_cycle - 1) % 40, (current_cycle - 1) / 40);

        if crt_x == 0 {
            ans_2.push('\n');
        }

        ans_2.push(match register.abs_diff(crt_x) < 2 {
            true => '#',
            false => '.',
        });

        current_cycle += 1;

        if let Some(x) = pending {
            register += x;
            pending = None;
        } else {
            // consume next command
            let line = commands.next().unwrap();
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens[0] == "addx" {
                let x = tokens[1].parse().unwrap();
                pending = Some(x);
            }
        }
    }

    println!("Answer 1: {}", ans_1);
    println!("Answer 2: {}", ans_2);
}
