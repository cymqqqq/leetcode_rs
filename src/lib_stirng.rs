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
