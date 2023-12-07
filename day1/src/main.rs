use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILEPATH: &str = "input.txt";

fn main() {
    let file = File::open(FILEPATH).expect(format!("File does not exists {FILEPATH}").as_str());
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let res = calculate(&lines);

    println!("RESULT -> {res}")
}

// we have a line
// line can have numbers as 0-9 and numbers as one-nine
// find the first and the last number in the line
// for the each this par of first/last find a sum of each of these pairs in array of lines
fn calculate(lines: &Vec<String>) -> u32 {
    let number_map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut result: u32 = 0;
    for line in lines {
        let mut min_position = line.len();
        let mut max_position = 0;

        let mut first_number = "";
        let mut last_number = "";
        // basically keep track of lowest and heights indexes in the line
        // for last we need to use rfind, because find returns first appearance
        for number_as_str in &number_map {
            if let Some(pos) = line.find(number_as_str.0) {
                if pos <= min_position {
                    min_position = pos;
                    first_number = number_as_str.1;
                }
            }

            if let Some(pos) = line.rfind(number_as_str.0) {
                if pos >= max_position {
                    max_position = pos;
                    last_number = number_as_str.1;
                }
            }
        }

        let concated = format!("{first_number}{last_number}");
        result += concated.parse::<u32>().unwrap();
        println!("LINE {line}; first-number {first_number} last number -> {last_number} -> result {result}");
    }

    result
}
