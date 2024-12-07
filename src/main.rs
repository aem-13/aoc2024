use aoc::day01;
use aoc::day02;
use aoc::day03;
use aoc::utils::parse;

fn main() {
    let day01_path = "input/day01.txt";
    let input = match parse::parse_columns_from_file(&day01_path) {
        Ok(res) => res,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    println!("Part One: {}", day01::part_one(&input));
    println!("Part Two: {}", day01::part_two(&input));

    let day02_path = String::from("input/day02.txt");
    let input = match day02::parse(day02_path){
        Ok(res)=>res,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    println!("Part One: {}", day02::part_one(&input));
    println!("Part Two: {}", day02::part_two(&input));

    let day03_path = String::from("input/day03.txt");
    let input = match day02::parse(day03_path){
        Ok(res)=>res,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    println!("Part One: {}", day03::part_one(&input));
    println!("Part Two: {}", day03::part_two(&input));
}
