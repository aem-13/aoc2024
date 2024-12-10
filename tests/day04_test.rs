use aoc::day04::*;



const EXAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[test]
fn part_one_test(){
    let input = Grid::new(String::from(EXAMPLE));
    assert_eq!(part_one(&input), 18);
}

#[test]
fn part_two_test(){
    let input = Grid::new(String::from(EXAMPLE));
    assert_eq!(part_two(&input), 9);
}