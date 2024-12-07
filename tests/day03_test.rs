use aoc::day03::*;

const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[test]
fn part_one_test() {
    let input = String::from(EXAMPLE);
    assert_eq!(part_one(&input), 161);
}

#[test]
fn part_two_test() {
    let input = String::from(EXAMPLE2);
    assert_eq!(part_two(&input), 48);
}