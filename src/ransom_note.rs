/*
Ransom Note
Given an arbitrary ransom note string and another string containing letters from all the magazines, write a function that will return true if the ransom note can be constructed from the magazines ; otherwise, it will return false.

Each letter in the magazine string can only be used once in your ransom note.

 

Example 1:

Input: ransomNote = "a", magazine = "b"
Output: false
Example 2:

Input: ransomNote = "aa", magazine = "ab"
Output: false
Example 3:

Input: ransomNote = "aa", magazine = "aab"
Output: true
 

Constraints:

You may assume that both strings contain only lowercase letters.
*/
    
use std::collections::HashMap;
pub fn construct(ransom_node: String, magzaine: String) -> bool {
    let mut hm: HashMap<char, i32> = HashMap::new();
    if ransom_node.len() > magzaine.len() {
        return false;
    }
    if ransom_node == magzaine {
        return true;
    }
    for c in magzaine.chars() {
        let e = hm.entry(c).or_default();
        *e += 1;
    }
    for c in ransom_node.chars() {
        if let Some(v) = hm.get_mut(&c) {
            *v -= 1;
            if *v < 0 { return false; }
            
        } else { return false; }
    }
    true
}
