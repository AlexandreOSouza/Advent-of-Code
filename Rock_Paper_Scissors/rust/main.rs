use std::collections::HashMap;
use std::fs;

fn modify_input(mut content: String) -> String {
    let mut curr_player = 1;
    let mut changes_draw = HashMap::new();
    changes_draw.insert("A", "X"); // Rock
    changes_draw.insert("B", "Y"); // Paper
    changes_draw.insert("C", "Z"); // Scissors

    let mut changes_win = HashMap::new();
    changes_win.insert("A", "Y"); // Rock
    changes_win.insert("B", "Z"); // Paper
    changes_win.insert("C", "X"); // Scissors

    let mut changes_lose = HashMap::new();
    changes_lose.insert("A", "Z"); // Rock
    changes_lose.insert("B", "X"); // Paper
    changes_lose.insert("C", "Y"); // Scissors

    let lines = content.lines();
    let mut result = String::new();

    for line in lines {
        let line1 = &line[..1];
        let line2 = &line[2..];

        if line2 == "Z" {
            let modify = changes_win.get(line1).unwrap();
            curr_player = 2;
            let next_result = format!("{} {}\n", line1, modify);
            result.push_str(&next_result);
        } else if line2 == "Y" {
            let modify = changes_draw.get(line1).unwrap();
            curr_player = 3;
            let next_result = format!("{} {}\n", line1, modify);
            result.push_str(&next_result);
        } else {
            let modify = changes_lose.get(line1).unwrap();
            curr_player = 1;
            let next_result = format!("{} {}\n", line1, modify);
            result.push_str(&next_result);
        }
    }

    return result;
}

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
    let mut content = fs::read_to_string(filepath).expect("Error trying to read the file");

    content = modify_input(content);

    let lines = content.lines();

    for line in lines {
        let line1 = &line[..1];
        let line2 = &line[2..];

        //        println!("{} {}", line1, line2);
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
