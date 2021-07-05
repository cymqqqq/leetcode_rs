/*
Repeated Substring Pattern
Given a non-empty string check if it can be constructed by taking a substring of it and appending multiple copies of the substring together. You may assume the given string consists of lowercase English letters only and its length will not exceed 10000.

 

Example 1:

Input: "abab"
Output: True
Explanation: It's the substring "ab" twice.
Example 2:

Input: "aba"
Output: False
Example 3:

Input: "abcabcabcabc"
Output: True
Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)
*/
pub fn repeat_subsequence_pattern(s: String) -> bool {
    let mut t: String = "".to_string();
    let n = s.len();
    if n < 2 { return false; }
    t += &s;
    println!("{:?}", t);
    t += &s;
    println!("{:?}", t);
    t[1..2 * n - 1].contains(&s)
}
fn main() {
    let s = "abc".to_string();
    println!("{:?}", repeat_subsequence_pattern(s));
}
