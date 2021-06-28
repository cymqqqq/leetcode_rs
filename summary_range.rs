/*
Summary Ranges
You are given a sorted unique integer array nums.

Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.

Each range [a,b] in the list should be output as:

"a->b" if a != b
"a" if a == b
 

Example 1:

Input: nums = [0,1,2,4,5,7]
Output: ["0->2","4->5","7"]
Explanation: The ranges are:
[0,2] --> "0->2"
[4,5] --> "4->5"
[7,7] --> "7"
Example 2:

Input: nums = [0,2,3,4,6,8,9]
Output: ["0","2->4","6","8->9"]
Explanation: The ranges are:
[0,0] --> "0"
[2,4] --> "2->4"
[6,6] --> "6"
[8,9] --> "8->9"
Example 3:

Input: nums = []
Output: []
Example 4:

Input: nums = [-1]
Output: ["-1"]
Example 5:

Input: nums = [0]
Output: ["0"]

*/
use std::fmt;
pub struct Range {
    start: i32,
    end: i32,
}
impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.start == self.end {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}->{}", self.start, self.end)
        }
    }
}
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut r: Option<Range> = None;
    let mut res: Vec<String> = vec![];
    for i in nums {
        if let Some(prev) = r {
            if prev.end + 1 == i {
                r = Some(Range {
                    start: prev.start,
                    end: i,
                });
                continue;
            } else {
                res.push(format!("{}", prev));
                r = Some(Range { start: i, end: i});
            }
        } else {
            r = Some(Range { start: i, end: i})
        }
    }
    if let Some(last) = r {
        res.push(format!("{}", last));
    }
    res
}
fn main() {
    let num = vec![0,2,4,5,8];
    println!("{:?}", summary_ranges(num));
}
