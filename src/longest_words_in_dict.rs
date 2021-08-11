/*
Longest Word in Dictionary
Given a list of strings words representing an English Dictionary, find the longest word in words that can be built one character at a time by other words in words. If there is more than one possible answer, return the longest word with the smallest lexicographical order.

If there is no answer, return the empty string.
Example 1:
Input: 
words = ["w","wo","wor","worl", "world"]
Output: "world"
Explanation: 
The word "world" can be built one character at a time by "w", "wo", "wor", and "worl".
Example 2:
Input: 
words = ["a", "banana", "app", "appl", "ap", "apply", "apple"]
Output: "apple"
Explanation: 
*/
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Trie {
    child: BTreeMap<char, Trie>,
    end: bool,
}
impl Trie {
    fn new() -> Self {
        Self::default()
    }
    fn from_words(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for word in words {
            trie.insert(word);
        }
        trie
    }
    fn insert(&mut self, s: String) {
        let mut link = self;
        for c in s.chars() {
            link = link.child.entry(c).or_default();
        }
        link.end = true;
    }
    fn dfs(&self, s: &mut String, max: &mut String) {
        if self.end {
            if s.len() > max.len() {
                *max = s.clone();
            }
        }
        if s.is_empty() || self.end {
            for (&c, child) in self.child.iter() {
                s.push(c);
                child.dfs(s, max);
                s.pop();
            }
        }
    }
}
pub fn longest_word(words: Vec<String>) -> String {
    let trie = Trie::from_words(words);
    let mut s: String = "".to_string();
    let mut max: String = "".to_string();
    trie.dfs(&mut s, &mut max);
    max
}
