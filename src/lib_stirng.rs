/*
is nagram

*/
pub fn is_anagram(s: String, t: String) -> bool {
    let mut s: Vec<char> = s.chars().collect();
    let mut t: Vec<char> = t.chars().collect();
    s.sort_unstable();
    t.sort_unstable();
    s == t
}
/*
shorest word distance
*/
use std::collections::HashMap;
use std::i32;
pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
    let mut pos: HashMap<String, Vec<i32>> = HashMap::new();
    let mut min = i32::MAX;
    for i in 0..words.len() {
        let w = &words[i];
        if let Some(v) = pos.get_mut(w) {
            v.push(i as i32);
        } else {
            pos.insert(w.clone(), vec![i as i32]);
        }
    }
    let v1 = pos.get(&word1).unwrap();
    let v2 = pos.get(&word2).unwrap();
    for k in v1 {
        for h in v2 {
            min = i32::min(min, (k - h).abs());
        }
    }
    min
}
fn main() {
    let words: Vec<String> = vec!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "practice".to_string();
    let word2 = "makes".to_string();
    println!("{:?}", shortest_distance(words, word1, word2));
}