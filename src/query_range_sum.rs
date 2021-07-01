/*
Range Sum Query - Immutable
Given an integer array nums, find the sum of the elements between indices i and j (i â‰¤ j), inclusive.

Implement the NumArray class:

NumArray(int[] nums) Initializes the object with the integer array nums.
int sumRange(int i, int j) Return the sum of the elements of the nums array in the range [i, j] inclusive (i.e., sum(nums[i], nums[i + 1], ... , nums[j]))
 

Example 1:

Input
["NumArray", "sumRange", "sumRange", "sumRange"]
[[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
Output
[null, 1, -1, -3]

Explanation
NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1)) 
numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))
 

Constraints:

0 <= nums.length <= 104
-105 <= nums[i] <= 105
0 <= i <= j < nums.length
At most 104 calls will be made to sumRange.
*/
pub struct Array {
    prefix_sum: Vec<i32>,
}
impl Array {
    fn new(mut num: Vec<i32>) -> Self {
        for i in 1..num.len() {
            num[i] += num[i - 1];
        }
        Array {
            prefix_sum: num
        }
    }
    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let j: usize = j as usize;
        let i: usize = i as usize;
        if i == 0 {
            self.prefix_sum[j]
        } else {
            self.prefix_sum[j] - self.prefix_sum[i - 1]
        }
    }
}
