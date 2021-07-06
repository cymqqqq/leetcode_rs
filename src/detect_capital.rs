/*
Detect Capital
Given a word, you need to judge whether the usage of capitals in it is right or not.

We define the usage of capitals in a word to be right when one of the following cases holds:

All letters in this word are capitals, like "USA".
All letters in this word are not capitals, like "leetcode".
Only the first letter in this word is capital, like "Google".
Otherwise, we define that this word doesn't use capitals in a right way.
 

Example 1:

Input: "USA"
Output: True
 

Example 2:

Input: "FlaG"
Output: False
*/
pub fn detect_capital_use(word: String) -> bool {
    let n = word.len();
    if n <= 1 { return true; }
    let word: Vec<char> = word.chars().collect();
    let first_is_lowercase: bool = word[0].is_lowercase();
    if first_is_lowercase {
        for i in 1..n {
            if word[i].is_uppercase() { return false; }
        }
    } else {
        let mut prev: Option<bool> = None;
        for i in 1..n {
            if let Some(prev_case) = prev {
                if prev_case != word[i].is_uppercase() {
                    return false;
                }
            } else {
                prev = Some(word[i].is_uppercase());
            }
        }
    }
    true
}
