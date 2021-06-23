/*
Missing Ranges
You are given an inclusive range [lower, upper] and a sorted unique integer array nums, where all elements are in the inclusive range.

A number x is considered missing if x is in the range [lower, upper] and x is not in nums.

Return the smallest sorted list of ranges that cover every missing number exactly. That is, no element of nums is in any of the ranges, and each missing number is in one of the ranges.

Each range [a,b] in the list should be output as:

"a->b" if a != b
"a" if a == b
 

Example 1:

Input: nums = [0,1,3,50,75], lower = 0, upper = 99
Output: ["2","4->49","51->74","76->99"]
Explanation: The ranges are:
[2,2] --> "2"
[4,49] --> "4->49"
[51,74] --> "51->74"
[76,99] --> "76->99"
Example 2:

Input: nums = [], lower = 1, upper = 1
Output: ["1"]
Explanation: The only missing range is [1,1], which becomes "1".
Example 3:

Input: nums = [], lower = -3, upper = -1
Output: ["-3->-1"]
Explanation: The only missing range is [-3,-1], which becomes "-3->-1".
Example 4:

Input: nums = [-1], lower = -1, upper = -1
Output: []
Explanation: There are no missing ranges since there are no missing numbers.
Example 5:

Input: nums = [-1], lower = -2, upper = -1
Output: ["-2"]
 

Constraints:

-109 <= lower <= upper <= 109
0 <= nums.length <= 100
lower <= nums[i] <= upper
All the values of nums are unique.
*/
use std::fmt;
struct Interval(i64, i64);
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == self.1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{} -> {}", self.0, self.1)
        }
    }
}
pub fn find_miss_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
    let mut cur = Interval(lower as i64, upper as i64);
    let mut res: Vec<Interval> = vec![];
    for i in nums {
        let i = i as i64;
        if i > cur.0 {
            res.push(Interval(cur.0, i - 1));
        }
        cur.0 = i + 1;
    }
    if cur.0 <= cur.1 {
        res.push(cur);
    }
    res.iter().map(|x| x.to_string()).collect()
}
fn main() {
    let nums = vec![0,1,3,59,100];
    let lower = 0;
    let upper = 100;
    println!("{:?}", find_miss_ranges(nums, lower, upper));
}
