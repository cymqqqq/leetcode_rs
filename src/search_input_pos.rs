/*
Search Insert Position
Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2
Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1
Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4
Example 4:

Input: nums = [1,3,5,6], target = 0
Output: 0
*/
//solution 1
pub fn search_insert_pos(nums: &mut Vec<i32>, tar: i32) -> i32 {
    let end = nums.len() - 1;
    for i in 0..end {
        if tar <= nums[i] {
            return i as i32;
        }
    }
    return end as i32;
}
//solution 2
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
fn main() {
    let mut a = vec![1,3,5,6];
    println!("{:?}", search_insert_pos(&mut a, 0));
}
