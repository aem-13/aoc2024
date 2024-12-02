use aoc::day01::day01;
use aoc::utils::parse;

fn main() {
    let day01_path = "input/day01.txt";
    let input = match parse::parse_columns_from_file(&day01_path) {
        Ok(res) => res,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    println!("Part One: {}", day01::part_one(&input));
    println!("Part Two: {}", day01::part_two(&input));
}
