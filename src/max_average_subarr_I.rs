/*
Maximum Average Subarray I
Given an array consisting of n integers, find the contiguous subarray of given length k that has the maximum average value. And you need to output the maximum average value.

Example 1:

Input: [1,12,-5,-6,50,3], k = 4
Output: 12.75
Explanation: Maximum average is (12-5-6+50)/4 = 51/4 = 12.75
*/
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;
    let k = k as usize;
    let n = nums.len();
    for i in 0..k {
        sum += nums[i];
    }
    let mut max = sum;
    for i in k..n {
        sum += nums[i];
        sum -= nums[i - k];
        max = i32::max(sum, max);
    }
    max as f64 / k as f64
}
