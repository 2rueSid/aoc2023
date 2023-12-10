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
            let color = &cap[2];
            let number = &cap[1].parse::<i32>().expect("Number");

            if color == red.0 && number > &red.1 {
                passed = false;
                break;
            } else if color == green.0 && number > &green.1 {
                passed = false;
                break;
            } else if color == blue.0 && number > &blue.1 {
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

fn task2(games: &Vec<String>) -> u32 {
    let red = "red";
    let green = "green";
    let blue = "blue";

    let get_color = Regex::new(r"(\d+)\s*(blue|red|green)").unwrap();

    let mut res: u32 = 0;

    for game in games {
        let mut max_red: u32 = 1;
        let mut max_green: u32 = 1;
        let mut max_blue: u32 = 1;

        for cap in get_color.captures_iter(game) {
            let color = &cap[2];
            let number = cap[1].parse::<u32>().expect("Number");

            if color == red && number > max_red {
                max_red = number;
            } else if color == green && number > max_green {
                max_green = number;
            } else if color == blue && number > max_blue {
                max_blue = number;
            }
        }

        res += max_red * max_blue * max_green;
    }

    res
}

fn main() {
    let file = File::open(FILEPATH).expect(format!("File does not exists {FILEPATH}").as_str());
    let reader = BufReader::new(file);

    let games: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let res1 = task1(&games);
    println!("RESULT FOR TASK 1 -> {res1}");

    let res2 = task2(&games);
    println!("RESULT FOR TASK 2 -> {res2}");
}
