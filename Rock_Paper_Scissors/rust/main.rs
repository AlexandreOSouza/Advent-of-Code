use std::collections::HashMap;
use std::fs;

fn main() {
    let mut player1 = HashMap::new();
    player1.insert("A", 1); // Rock
    player1.insert("B", 2); // Paper
    player1.insert("C", 3); // Scissors

    let mut player2 = HashMap::new();
    player2.insert("X", 1); // Rock
    player2.insert("Y", 2); // Paper
    player2.insert("Z", 3); // Scissors

    let mut win = HashMap::new();
    win.insert(1, 3);
    win.insert(2, 1);
    win.insert(3, 2);

    let mut my_points = 0;

    let filepath: &str = "../input.txt";
    let content = fs::read_to_string(filepath).expect("Error trying to read the file");

    let lines = content.lines();

    for line in lines {
        let line1 = &line[..1];
        let line2 = &line[2..];
        let play1 = player1.get(line1).unwrap();
        let play2 = player2.get(line2).unwrap();

        if play1 == play2 {
            my_points += play2 + 3;
        } else {
            let play1_win = win.get(play1).unwrap() == play2;
            if play1_win {
                my_points += play2;
            } else {
                my_points += play2 + 6;
            }
        }
    }
    println!("My points: {}", my_points);
}
