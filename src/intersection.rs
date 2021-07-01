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
pub fn intersectin_II(num1: Vec<i32>, num2: Vec<i32>) -> Vec<i32> {
    let mut hm1: HashMap<i32, i32> = HashMap::new();
    let hs1: HashSet<i32> = num1.clone().into_iter().collect();
    num1.iter().for_each(|&x| {
        *hm1.entry(x).or_default() += 1;
    });
    let mut hm2: HashMap<i32, i32> = HashMap::new();
    let hs2: HashSet<i32> = num2.clone().into_iter().collect();
    num2.iter().for_each(|&x| {
        *hm2.entry(x).or_default() += 1;
    });
    let bit = &hs1 & &hs2;
    let mut res: Vec<i32> = vec![];
    bit.iter().for_each(|&x| {
        let c1 = hm1[&x];
        let c2 = hm2[&x];
        let min = i32::min(c1, c2);
        (0..min).for_each(|_| {
            res.push(x);
        });
    });
    res
}
fn main() {
    let num1 = vec![1,2,3,4];
    let num2 = vec![3];
    println!("{:?}", intersection(num1, num2));
}
