use std::fs;

fn main() {
    let filename: &str = "../input.txt";

    let content = fs::read_from_string(filename).expect("Error trying to read the file");
}
