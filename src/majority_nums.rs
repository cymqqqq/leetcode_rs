/*
Majority Element
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than âŒŠn / 2âŒ‹ times. You may assume that the majority element always exists in the array.

 

Example 1:

Input: nums = [3,2,3]
Output: 3
Example 2:

Input: nums = [2,2,1,1,1,2,2]
Output: 2
 

Constraints:

n == nums.length
1 <= n <= 5 * 104
-231 <= nums[i] <= 231 - 1
*/
pub fn majority(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut num: Option<i32> = None;
    for i in nums {
        if let Some(n) = num {
            if i == n {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    num = None;
                }
            }
        } else {
            num = Some(i);
            count += 1;
        }
    }
    num.unwrap()
}
fn main() {
    let nums = vec![2,2,1,1,1,1,2];
    println!("{:?}", majority(nums));
}
