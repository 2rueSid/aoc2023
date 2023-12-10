use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILEPATH: &str = "input.txt";

fn task1(games: &Vec<String>) -> u32 {
    let red = ("red", 12);
    let green = ("green", 13);
    let blue = ("blue", 14);

    let get_color = Regex::new(r"(\d+)\s*(blue|red|green)").unwrap();

    let mut res: u32 = 0;
    let mut game_count = 0;

    for game in games {
        game_count += 1;
        let mut passed = true;
        for cap in get_color.captures_iter(game) {
            if &cap[2] == red.0 && &cap[1].parse::<i32>().expect("Number") > &red.1 {
                passed = false;
                break;
            } else if &cap[2] == green.0 && &cap[1].parse::<i32>().expect("Number") > &green.1 {
                passed = false;
                break;
            } else if &cap[2] == blue.0 && &cap[1].parse::<i32>().expect("Number") > &blue.1 {
                passed = false;
                break;
            }
        }
        println!("{game} -> PASSED {passed}");
        if passed {
            res += game_count;
        }
    }

    res
}

fn main() {
    let file = File::open(FILEPATH).expect(format!("File does not exists {FILEPATH}").as_str());
    let reader = BufReader::new(file);

    let games: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let res1 = task1(&games);
    println!("RESULT FOR TASK 1 -> {res1}");
}
