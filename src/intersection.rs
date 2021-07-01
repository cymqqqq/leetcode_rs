/*
Intersection of Two Arrays
Given two arrays, write a function to compute their intersection.

Example 1:

Input: nums1 = [1,2,2,1], nums2 = [2,2]
Output: [2]
Example 2:

Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
Output: [9,4]
Note:

Each element in the result must be unique.
The result can be in any order.
*/
use std::collections::HashSet;
pub fn intersection(num1: Vec<i32>, num2: Vec<i32>) -> Vec<i32> {
    let hs1: HashSet<i32> = num1.into_iter().collect();
    let hs2: HashSet<i32> = num2.into_iter().collect();
    let bit = &hs1 & &hs2;
    bit.into_iter().collect()
}
fn main() {
    let num1 = vec![1,2,3,4];
    let num2 = vec![3];
    println!("{:?}", intersection(num1, num2));
}
