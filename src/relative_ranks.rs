/*
Relative Ranks
Given scores of N athletes, find their relative ranks and the people with the top three highest scores, who will be awarded medals: "Gold Medal", "Silver Medal" and "Bronze Medal".

Example 1:
Input: [5, 4, 3, 2, 1]
Output: ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
Explanation: The first three athletes got the top three highest scores, so they got "Gold Medal", "Silver Medal" and "Bronze Medal". 
For the left two athletes, you just need to output their relative ranks according to their scores.
Note:
N is a positive integer and won't exceed 10,000.
All the scores of athletes are guaranteed to be unique.
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Athlete {
    idx: usize,
    score: i32,
    rank: String,
}
pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
    let n = nums.len();
    let mut a: Vec<Athlete> = vec![];
    for i in 0..n {
        a.push(Athlete {
            idx: i,
            score: nums[i],
            rank: "".to_string(),
        });
        
    }
    a.sort_unstable_by(|a, b| b.score.cmp(&a.score));
    for i in 0..n {
        a[i].rank = match i {
            0 => "gold medal".to_string(),
            1 => "silver medal".to_string(),
            2 => "bronze medal".to_string(),
            _ => format!("{}", i + 1),
        }
    }
    a.sort_unstable_by(|a, b| a.idx.cmp(&b.idx));
    a.into_iter().map(|a| a.rank).collect()
}
