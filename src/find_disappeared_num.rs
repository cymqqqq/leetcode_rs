/*
Find All Numbers Disappeared in an Array
Given an array of integers where 1 â‰¤ a[i] â‰¤ n (n = size of array), some elements appear twice and others appear once.

Find all the elements of [1, n] inclusive that do not appear in this array.

Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

Example:

Input:
[4,3,2,7,8,2,3,1]

Output:
[5,6]
*/
pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        let idx: usize = (nums[i].abs() as usize) - 1;
        nums[idx] = -nums[idx].abs();
    }
    let mut res: Vec<i32> = vec![];
    for i in 1..=n {
        let idx = i - 1;
        if nums[idx] > 0 {
            res.push(i as i32);
        }
    }
    res
}
