use regex::Regex;
use std::error::Error;
use std::fs;

type Input = String;

pub fn parse(path: String) -> Result<String, Box<dyn Error>> {
    let data: String = fs::read_to_string(path)?;
    Ok(data)
}

pub fn part_one(input: &Input) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| {
            c.get(1)
                .map(|m| m.as_str())
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap()
                * c.get(2)
                    .map(|m| m.as_str())
                    .unwrap_or("0")
                    .parse::<i64>()
                    .unwrap()
        })
        .sum()
}

pub fn part_two(input: &Input) -> i64 {
    let re = Regex::new(r"do\(\)|mul\((\d+),(\d+)\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    for cap in re.captures_iter(&input) {
        match cap.get(0).map(|m| m.as_str()) {
            Some("don't()") => {
                enabled = false;
            }
            Some("do()") => {
                enabled = true;
            }
            Some(_) => {
                if enabled {
                    sum += cap
                        .get(1)
                        .map(|m| m.as_str())
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap()
                        * cap
                            .get(2)
                            .map(|m| m.as_str())
                            .unwrap_or("0")
                            .parse::<i64>()
                            .unwrap();
                }
            }
            None => println!("No good"),
        }
    }
    sum
}
