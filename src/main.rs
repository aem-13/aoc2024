use aoc::utils::parse;
use aoc::day01::day01;

fn main() {
    
    let day01_p1_path = "input/day01.txt";
    let input_res = parse::parse_columns_from_file(&day01_p1_path);
    let input = match input_res{
        Ok(res) => res,
        Err(error) => panic!("Problem opening the file: {error:?}"),
        };

    println!("Part One: {}", day01::part_one(&input));
    println!("Part Two: {}", day01::part_two(&input));

}

