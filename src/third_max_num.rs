/*
Third Maximum Number
Given integer array nums, return the third maximum number in this array. If the third maximum does not exist, return the maximum number.

 

Example 1:

Input: nums = [3,2,1]
Output: 1
Explanation: The third maximum is 1.
Example 2:

Input: nums = [1,2]
Output: 2
Explanation: The third maximum does not exist, so the maximum (2) is returned instead.
Example 3:

Input: nums = [2,2,3,1]
Output: 1
Explanation: Note that the third maximum here means the third maximum distinct number.
Both numbers with value 2 are both considered as second maximum.
 
*/
use std::cmp::Ordering::*;
pub fn third_max(nums: Vec<i32>) -> i32 {
    let mut max0 = nums[0];
    let mut m1: Option<i32> = None;
    let mut m2: Option<i32> = None;
    for i in nums{
        match i.cmp(&max0) {
            Greater => {
                m2 = m1;
                m2 = Some(max0);
                max0 = i;
            }
            Less => {
                if let Some(max1) = m1 {
                    match i.cmp(&max1) {
                        Greater => {
                            m2 = m1;
                            m1 = Some(i);
                        }
                        Less => {
                            if let Some(max2) = m2 {
                                if i > max2 {
                                    m2 = Some(i);
                                }
                            } else {
                                m2 = Some(i);
                            }
                        }
                        Equal => {}
                    }
                } else {
                    m1 = Some(i);
                }
            }
            Equal => {}
        }
    }
    if let Some(max2) = m2 {
        max2
    } else {
        max0
    }
}
