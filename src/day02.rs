use std::error::Error;
use std::fs;

type Input = String;

pub fn parse(path: String) -> Result<String, Box<dyn Error>> {
    let data: String = fs::read_to_string(path)?;
    Ok(data)
}

pub fn part_one(input: &Input) -> u32 {
    let mut report: Vec<i32> = Vec::new();
    let mut safe: u32 = 0;
    for line in input.lines() {
        report.extend(
            line.split_whitespace()
                .map(|s: &str| s.parse::<i32>().unwrap()),
        );

        let inc = check(&report);
        safe += inc;
        report.clear();
    }

    return safe;
}

pub fn part_two(input: &Input) -> u32 {
    let mut report: Vec<i32> = Vec::new();
    let mut safe: u32 = 0;
    for line in input.lines() {
        report.extend(
            line.split_whitespace()
                .map(|s: &str| s.parse::<i32>().unwrap()),
        );

        let size = report.len();
        let inc = check(&report);
        
        if inc == 1 {
            safe += 1;
        } else {
            for i in 0..size{
                let mut reduced_report = report.clone();
                reduced_report.remove(i);
                
                if 1 == check(&reduced_report) {
                    safe +=1;
                    break;
                }
                
            }
        };
        report.clear();
    }
    return safe;
}

fn check(report: &[i32]) -> u32 {
    let deltas: Vec<i32> = report.windows(2).map(|p| p[1] - p[0]).collect();

    let mut prev = 0;
    for i in deltas.into_iter() {
        if i.abs() > 3 {
            return 0;
        }
        if i == 0 {
            return 0;
        }
        if prev * i < 0 {
            return 0;
        }
        prev = i;
    }
    return 1;
}
