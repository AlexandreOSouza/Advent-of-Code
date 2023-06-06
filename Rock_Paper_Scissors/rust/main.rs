use std::fs;

fn main() {
    let filepath: &str = "../input.txt";
    let content = fs::read_to_string(filepath).expect("Error trying to read the file");

    println!("{}", content);
}
