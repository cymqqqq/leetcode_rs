/*
Isomorphic Strings
Given two strings s and t, determine if they are isomorphic.

Two strings s and t are isomorphic if the characters in s can be replaced to get t.

All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

 

Example 1:

Input: s = "egg", t = "add"
Output: true
Example 2:

Input: s = "foo", t = "bar"
Output: false
Example 3:

Input: s = "paper", t = "title"
Output: true
 

Constraints:

1 <= s.length <= 5 * 104
t.length == s.length
s and t consist of any valid ascii character.
*/
use std::collections::HashMap;
pub fn isomorphic(s: String, t: String) -> bool {
    let mut ss: HashMap<char, char> = HashMap::new();
    let mut tt: HashMap<char, char> = HashMap::new();
    let mut is = s.chars();
    let mut it = t.chars();
    while let (Some(vs), Some(vt)) = (is.next(), it.next()) {
        if let Some(&cs) = ss.get(&vs) {
            if cs != vt { return false; }
            else { ss.insert(vs, vt); }
        }
        if let Some(&ct) = tt.get(&vt) {
            if ct != vs { return false; }
            else { tt.insert(ct, vs); }
        }
    }
    true
}
fn main() {
    let s = "cai".to_string();
    let t = "ccc".to_string();
    
    println!("{:?}", isomorphic(s, t));
}
