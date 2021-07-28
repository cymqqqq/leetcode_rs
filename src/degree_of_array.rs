/*
Degree of an Array
Given a non-empty array of non-negative integers nums, the degree of this array is defined as the maximum frequency of any one of its elements.

Your task is to find the smallest possible length of a (contiguous) subarray of nums, that has the same degree as nums.

 

Example 1:

Input: nums = [1,2,2,3,1]
Output: 2
Explanation: 
The input array has a degree of 2 because both elements 1 and 2 appear twice.
Of the subarrays that have the same degree:
[1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
The shortest length is 2. So return 2.
Example 2:

Input: nums = [1,2,2,3,1,4,2]
Output: 6
Explanation: 
The degree is 3 because the element 2 is repeated 3 times.
So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.

*/
pub struct Degree {
    left: usize,
    right: usize,
    count: usize,
}
use std::collections::HashMap;
use std::usize;
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, Degree> = HashMap::new();
    let n = nums.len();
    let mut max_degree: usize = 0;
    for i in 0..n {
        let a = nums[i];
        let e = hm.entry(a).or_insert(Degree {
            left: i,
            right: i,
            count: 0,
        });
        e.left = usize::min(e.left, i);
        e.right = usize::max(e.right, i);
        e.count += 1;
        max_degree = usize::max(e.count, max_degree);
    }
    let mut min_width: usize = n;
    for i in hm.values() {
        if i.count == max_degree {
            min_width = usize::min(i.right - i.left + 1, min_width);
        }
    }
    min_width as i32
}
