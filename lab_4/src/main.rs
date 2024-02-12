// use regex::Regex;
use std::fs;


fn main() {
    let input_path = std::path::Path::new("input.txt");
    let content = fs::read_to_string(&input_path)
        .expect("File not located or unable to be read.");
    let lines = content.lines();

    for line in lines {
        println!("{}", line);
    }
}
