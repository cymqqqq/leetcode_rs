pub fn palimdrome(num: i32) -> bool {
    if num < 0 { return false; }
    else {
        let x_str = num.abs().to_string();
        if x_str == x_str.chars().rev().collect::<String>() {
            return true;
        } else { return false; }
    }
    
}
/*
Valid Palindrome II
Given a non-empty string s, you may delete at most one character. Judge whether you can make it a palindrome.

Example 1:
Input: "aba"
Output: True
Example 2:
Input: "abca"
Output: True
Explanation: You could delete the character 'c'.
*/
pub fn valid_palindrome(s: String) -> bool {
    let v: &str = s.as_str();
    if let Some(s) = is_palindrome(v) {
        let sl: &str = &s[1..];
        let sr: &str = &s[..s.len() - 1];
        is_palindrome(sl).is_none() || is_palindrome(sr).is_none()
    } else { true }
}
pub fn is_palindrome(s: &str) -> Option<&str> {
    let n = s.len();
    if n <= 1 { return None; }
    if s.chars().next() != s.chars().last() {
        Some(s)
    } else {
        is_palindrome(&s[1..n - 1])
    }
}
///////////
fn main() {
    let x = 2120;
    println!("{:?}", palimdrome(x));
}
