use aoc::day01;

#[test]
fn part_one_test() {
    let input1 = vec![3, 4, 2, 1, 3, 3];
    let input2 = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(11, day01::part_one(&(input1, input2)))
}
#[test]
fn part_two_test(){
    let input1 = vec![3, 4, 2, 1, 3, 3];
    let input2 = vec![4, 3, 5, 3, 9, 3];
    assert_eq!(31, day01::part_two(&(input1, input2)))
}
