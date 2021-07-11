/*
Maximum Product of Three Numbers
Given an integer array nums, find three numbers whose product is maximum and return the maximum product.

 

Example 1:

Input: nums = [1,2,3]
Output: 6
Example 2:

Input: nums = [1,2,3,4]
Output: 24
Example 3:

Input: nums = [-1,-2,-3]
Output: -6
*/
pub fn maximum_product(mut num: Vec<i32>) -> i32 {
    num.sort_unstable();
    let n = num.len();
    i32::max(
        num[0] * num[1] * num[n - 1],
        num[n - 1] * num[n - 2] * num[n - 3],
    )
}
fn main() {
    let num = vec![1,2,3];
    println!("{:?}", maximum_product(num));
}
