/*
Longest Harmonious Subsequence
We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.

Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.

A subsequence of array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements.

 

Example 1:

Input: nums = [1,3,2,2,5,2,3,7]
Output: 5
Explanation: The longest harmonious subsequence is [3,2,2,2,3].
Example 2:

Input: nums = [1,2,3,4]
Output: 2
*/
use std::collections::HashMap;
pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let mut hs: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for &x in &nums {
        let e = hs.entry(x).or_default();
        *e += 1;
    }
    for (x, u) in &hs {
        if let Some(v) = hs.get(&(x - 1)) {
            max = i32::max(u + v, max);
        }
    }
    max
}
