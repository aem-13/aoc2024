use std::{collections::HashMap, vec::Vec};
pub fn part_one((input1, input2): &(Vec<i32>, Vec<i32>)) -> i32 {
    let len1 = input1.len();
    let len2 = input2.len();
    if len1 != len2 {
        return 0;
    }
    let mut left = input1.clone();
    let mut right = input2.clone();
    left.sort();
    right.sort();

    let mut cache1: HashMap<usize, i32> = HashMap::new();
    let mut cache2: HashMap<usize, i32> = HashMap::new();

    for (index, value) in left.into_iter().enumerate() {
        cache1.insert(index, value);
    }
    for (index, value) in right.into_iter().enumerate() {
        cache2.insert(index, value);
    }
    let mut sum = 0;
    for i in 0..len1 {
        let num: i32 = *(cache1.get(&i).unwrap()) - *(cache2.get(&i).unwrap());
        sum += num.abs()
    }

    return sum;
}

pub fn part_two((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
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
