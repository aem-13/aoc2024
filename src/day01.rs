use std::{collections::HashMap, vec::Vec};

type Input = (Vec<i32>, Vec<i32>);

pub fn part_one(input: &Input) -> i32 {
   
    let (mut left, mut right) = input.clone();
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len(){
        sum += (left[i]-right[i]).abs();
    }
    return sum;
}

pub fn part_two(input: &Input) -> i32 {
    let (left, right) = input;
    let mut freq_left: HashMap<i32, i32> = HashMap::with_capacity(1000);
    let mut freq_right: HashMap<i32, i32> = HashMap::with_capacity(1000);
    for elem in left.iter() {
        *freq_left.entry(*elem).or_insert(0) += 1;
    }
    for elem in right.iter() {
        *freq_right.entry(*elem).or_insert(0) += *elem;
    }
    let mut similarity = 0;
    for (key, value) in freq_left {
        let item = match freq_right.get(&key) {
            Some(v) => *v,
            None => 0,
        };
        similarity += value * item;
    }
    return similarity;
}
