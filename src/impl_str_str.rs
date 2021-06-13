/*
Implement strStr().

Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

Clarification:

What should we return when needle is an empty string? This is a great question to ask during an interview.

For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().
Example 1:

Input: haystack = "hello", needle = "ll"
Output: 2
Example 2:

Input: haystack = "aaaaa", needle = "bba"
Output: -1
*/
pub fn str_str(src: String, dst: String) -> i32 {
    match src.find(&dst) {
        Some(idx) => idx as i32,
        None => -1,
    }
}
fn main() {
    println!("{:?}", str_str(String::from("hello"), String::from("f")));
}
