use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day1-full.txt").expect("Could not read the input file");
    let mut calibration_sum = 0;
    let num_pattern =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|zero|[0-9]").unwrap();
    for line in input.lines() {
        if !line.is_empty() {
            let digits = line.chars().filter(|c| char::is_numeric(*c));
            let mut num_str: String = String::from(digits.clone().next().unwrap());
            num_str.push(digits.last().unwrap());
            calibration_sum += num_str.parse::<usize>().unwrap();
        }
    }

    println!("Calibration sum: {calibration_sum}");
}
