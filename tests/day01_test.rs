
use aoc::day01::day01;

#[test]
fn day01_test(){
    let mut input1 = vec![3,4,2,1,3,3];
    let mut input2 = vec![4,3,5,3,9,3];
    assert_eq!(11, day01::solution(&mut input1, &mut input2))
}