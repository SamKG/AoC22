use std::{fs::File, io::Read};

fn all_different<T: PartialEq>(v: &[T]) -> bool {
    for i in 0..v.len() {
        for j in i + 1..v.len() {
            if v[i] == v[j] {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let mut f = File::open("problem6/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut start_of_packet = vec![];

    for (idx, c) in buf.chars().enumerate() {
        start_of_packet.insert(0, c);
        if start_of_packet.len() > 4 {
            start_of_packet.pop();
        }
        if start_of_packet.len() == 4 && all_different(&start_of_packet) {
            println!("Answer Q1: {}", idx + 1);
            break;
        }
    }

    let mut start_of_message = vec![];

    for (idx, c) in buf.chars().enumerate() {
        start_of_message.insert(0, c);
        if start_of_message.len() > 14 {
            start_of_message.pop();
        }
        if start_of_message.len() == 14 && all_different(&start_of_message) {
            println!("Answer Q2: {}", idx + 1);
            break;
        }
    }
}
