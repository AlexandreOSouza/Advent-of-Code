use std::fs;

fn main() {
    let mut priority: Vec<u32> = Vec::new();
    const LOWER: u32 = 96;
    const UPPER: u32 = 38;
    let filename: &str = "../input.txt";

    let content = fs::read_to_string(filename).expect("Error trying to read the file");

    let lines = content.lines();
    for line in lines {
        let len = line.len();
        let half = len / 2;
        let first: Vec<char> = line[..half].chars().collect();
        let second = &line[half..];

        for x in first {
            if second.contains(x) {
                if x.is_lowercase() {
                    priority.push(x as u32 - LOWER);
                } else {
                    priority.push(x as u32 - UPPER);
                }
                break;
            }
        }
    }

    let sum: u32 = priority.iter().sum();
    println!("{}", sum);
}
