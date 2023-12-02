use std::{vec::Vec, fs};
use regex::Regex;

struct PullRecord {
    red: usize,
    green: usize,
    blue: usize
}

impl From<&str> for PullRecord {
    fn from(value: &str) -> Self {
        let mut ret = PullRecord { red: 0, green: 0, blue: 0 };
        let pull_re = Regex::new(r"(?P<count>\d+) (?P<color>red|blue|green)").unwrap();
        for color_pull in value.trim().split(", ") {
            let pull_caps = pull_re.captures(color_pull).expect("Unable to parse pull");
            match pull_caps.name("color").unwrap().as_str() {
                "red" => ret.red = pull_caps.name("count").unwrap().as_str().parse::<usize>().unwrap(),
                "blue" => ret.blue = pull_caps.name("count").unwrap().as_str().parse::<usize>().unwrap(),
                "green" => ret.green = pull_caps.name("count").unwrap().as_str().parse::<usize>().unwrap(),
                def => panic!("Unexpected color {def}")
            }
        }

        ret
    }
}

struct GameRecord {
    id: usize,
    pulls: Vec<PullRecord>
}

impl From<&str> for GameRecord {
    fn from(value: &str) -> Self {
        let mut ret = GameRecord { id: 0, pulls: Vec::new() };
        let game_re = Regex::new(r"Game (?P<game_id>\d+): (?P<pulls_str>.*)").unwrap();
        let game_captures = game_re.captures(value).expect("Unable to parse game");
        ret.id = game_captures.name("game_id").unwrap().as_str().parse::<usize>().unwrap();
        let pulls_str = game_captures.name("pulls_str").unwrap().as_str();
        for pull_str in pulls_str.split(";") {
            ret.pulls.push(PullRecord::from(pull_str));
        }
        ret
    }
}

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = fs::read_to_string("data/day2-full.txt").expect("Could not read input file");
    let mut possible_id_sum = 0;
    for line in input.lines() {
        let game = GameRecord::from(line);
        let impossible = game.pulls.iter().any(|pull| pull.red > MAX_RED || pull.green > MAX_GREEN || pull.blue > MAX_BLUE);
        if !impossible {
            possible_id_sum += game.id;
        }
    }

    println!("Possible id sum: {possible_id_sum}")
}
