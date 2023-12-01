use regex::Regex;
use std::fs;

fn convert_to_digit(to_convert: &str) -> usize {
    match to_convert {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        digit => digit.parse::<usize>().unwrap(),
    }
}

fn main() {
    let input = fs::read_to_string("data/day1-full.txt").expect("Could not read the input file");
    let mut calibration_sum = 0;
    let num_pattern =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|zero|[0-9]").unwrap();
    for line in input.lines() {
        if !line.is_empty() {
            let first_match = num_pattern.find(line).unwrap();
            let mut prev_match = first_match;
            let mut last_match = num_pattern.find_at(line, prev_match.start() + 1);
            while last_match.is_some() {
                prev_match = last_match.unwrap();
                last_match = num_pattern.find_at(line, prev_match.start() + 1);
            }
            let last_match = prev_match;
            let calibration_value =
                convert_to_digit(first_match.as_str()) * 10 + convert_to_digit(last_match.as_str());
            calibration_sum += calibration_value;
        }
    }

    println!("Calibration sum: {calibration_sum}");
}
