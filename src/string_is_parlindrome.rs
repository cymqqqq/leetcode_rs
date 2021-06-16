/*
Valid Palindrome
Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.

Note: For the purpose of this problem, we define empty string as valid palindrome.

Example 1:

Input: "A man, a plan, a canal: Panama"
Output: true
Example 2:

Input: "race a car"
Output: false
 

Constraints:

s consists only of printable ASCII characters.
*/
pub fn str_pralindrome(s: String) -> bool {
    let s: Vec<char> = s.chars()
                        .filter(|c| c.is_ascii_alphanumeric())
                        .map(|c| c.to_ascii_lowercase())
                        .collect();
    let a: String = s.iter().collect();
    let b: String = s.iter().rev().collect();
    a == b
}
fn main() {
    let price = String::from("A man, a plan, can not be this");
    println!("{:?}", str_pralindrome(price));
}
