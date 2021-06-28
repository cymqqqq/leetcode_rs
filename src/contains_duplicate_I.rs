/*
Contains Duplicate
Given an array of integers, find if the array contains any duplicates.

Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.

Example 1:

Input: [1,2,3,1]
Output: true
Example 2:

Input: [1,2,3,4]
Output: false
Example 3:

Input: [1,1,1,3,3,4,3,2,4,2]
Output: true
*/
use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset: HashSet<i32> = HashSet::new();
    for i in nums {
        if hashset.contains(&i) { return true; }
        else { hashset.insert(i); }
    }
    false
}
fn main() {
    let n = vec![1,2,4,3];
    println!("{:?}", contains_duplicate(n));
}
