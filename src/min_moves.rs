/*
Minimum Moves to Equal Array Elements
Given a non-empty integer array of size n, find the minimum number of moves required to make all array elements equal, where a move is incrementing n - 1 elements by 1.

Example:

Input:
[1,2,3]

Output:
3

Explanation:
Only three moves are needed (remember each move increments two elements):

[1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
*/
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut sum = 0;
    for &i in &nums {
        min = i32::min(i, min);
        println!("{:?}", min);
    }
    for &i in &nums {
        sum += i - min;
    }
    sum
}
fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", min_moves(nums));
}
