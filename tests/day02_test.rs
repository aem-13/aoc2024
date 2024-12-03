use aoc::day02::*;

const EXAMPLE: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[test]
fn part_one_test() {
    let input = String::from(EXAMPLE);
    assert_eq!(part_one(&input), 2);
}

#[test]
fn part2_test() {
    let input = String::from(EXAMPLE);
    assert_eq!(part_two(&input), 4);
}