/*
Palindrome Permutation
Given a string, determine if a permutation of the string could form a palindrome.

Example 1:

Input: 
"code"

Output: false
Example 2:

Input: 
"aab"

Output: true
Example 3:

Input: 
"carerac"

Output: true
*/
pub fn if_permute_palindrome(s: String) -> bool {
    let mut count: Vec<usize> = vec![0; 256];
    for i in s.bytes() {
        println!("{:?}", i);
        let n = i as usize;
        println!("{:?}", count[n]);
        if count[n] == 1 {
            count[n] = 0;
        } else {
            count[n] = 1;
        }
    }
    count.iter().sum::<usize>() <= 1
}
fn main() {
    println!("{:?}", if_permute_palindrome("code".to_string()));
}
