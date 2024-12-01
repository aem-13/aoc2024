use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_columns_from_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    
    for line in reader.lines() {
        let line = line?; 
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() >= 2 {
            if let (Ok(num1), Ok(num2)) = (columns[0].parse::<i32>(), columns[1].parse::<i32>()) {
                col1.push(num1);
                col2.push(num2);
            }
        }
    }

    Ok((col1, col2))
}