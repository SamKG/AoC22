use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("problem2/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let scores_1 = buf
        .lines()
        .map(|line| {
            let moves = line.split(' ').collect::<Vec<_>>();
            let move1 = moves[0];
            let move2 = moves[1];

            let winscore = match (move1, move2) {
                ("A", "X") => 3,
                ("A", "Y") => 6,
                ("A", "Z") => 0,
                ("B", "X") => 0,
                ("B", "Y") => 3,
                ("B", "Z") => 6,
                ("C", "X") => 6,
                ("C", "Y") => 0,
                ("C", "Z") => 3,
                _ => panic!("invalid move"),
            };
            let my_move_score = match move2 {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!("invalid move"),
            };

            winscore + my_move_score
        })
        .collect::<Vec<_>>();

    println!("Total score: {}", scores_1.iter().sum::<i32>());

    let scores_2 = buf
        .lines()
        .map(|line| {
            let moves = line.split(' ').collect::<Vec<_>>();
            let move1 = moves[0];
            let move2 = moves[1];

            let winscore = match (move2) {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("invalid move"),
            };

            let my_move_score = match (move1, move2) {
                ("A", "X") => 3,
                ("A", "Y") => 1,
                ("A", "Z") => 2,
                ("B", "X") => 1,
                ("B", "Y") => 2,
                ("B", "Z") => 3,
                ("C", "X") => 2,
                ("C", "Y") => 3,
                ("C", "Z") => 1,
                _ => panic!("invalid move"),
            };

            winscore + my_move_score
        })
        .collect::<Vec<_>>();

    println!("Total score: {}", scores_2.iter().sum::<i32>());
}
