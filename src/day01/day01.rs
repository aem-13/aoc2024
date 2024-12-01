
use std::{collections::HashMap, vec::Vec};
pub fn solution(input1: &mut Vec<i32>, input2: &mut Vec<i32>) -> i32{

    let len1 = input1.len();
    let len2 = input2.len();
    if len1 != len2{
        return 0;
    }

    input1.sort();
    input2.sort();

    let mut cache1: HashMap<usize,i32> = HashMap::new();
    let mut cache2: HashMap<usize,i32> = HashMap::new();

    for (index, value) in input1.into_iter().enumerate(){
        cache1.insert(index, *value);
      }
    for (index, value) in input2.into_iter().enumerate(){
        cache2.insert(index, *value);
    }
    let mut sum = 0;
    for i in 0..len1 {
        let num: i32 = *(cache1.get(&i).unwrap()) - *(cache2.get(&i).unwrap());
        sum += num.abs()
    }

    return sum

}