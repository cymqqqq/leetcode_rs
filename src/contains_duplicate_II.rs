/*
Contains Duplicate II
Given an array of integers and an integer k, find out whether there are two distinct indices i and j in the array such that nums[i] = nums[j] and the absolute difference between i and j is at most k.

Example 1:

Input: nums = [1,2,3,1], k = 3
Output: true
Example 2:

Input: nums = [1,0,1,1], k = 1
Output: true
Example 3:

Input: nums = [1,2,3,1,2,3], k = 2
Output: false
*/
use std::collections::HashSet;
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut hashset: HashSet<i32> = HashSet::new();
    let n = nums.len();
    let k = k as usize;
    for i in 0..n {
        let x = nums[i];
        if hashset.contains(&x) { return true; }
        else {
            hashset.insert(x);
            if hashset.len() > k {
                hashset.remove(&nums[i - k]);
            }
        }
    }
    false
}
fn main() {
    let n = vec![1,2,4,3];
    let k = 3;
    println!("{:?}", contains_nearby_duplicate(n, k));
}
