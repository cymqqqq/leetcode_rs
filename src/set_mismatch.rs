/*
Set Mismatch
You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.

You are given an integer array nums representing the data status of this set after the error.

Find the number that occurs twice and the number that is missing and return them in the form of an array.

 

Example 1:

Input: nums = [1,2,2,4]
Output: [2,3]
*/
use std::collections::HashSet;
pub fn find_nums(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut hs: HashSet<i32> = HashSet::new();
    let mut res: Vec<i32> = vec![];
    for i in nums {
        if !hs.insert(i) {
            res.push(i);
        }
    }
    for i in 1..=n {
        if hs.insert(i as i32) {
            res.push(i as i32);
        }
    }
    res
}
