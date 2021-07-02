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
/*
strobogrammatic number
*/
use std::collections::HashMap;
pub fn is_strobogrammatic(nums: String) -> bool {
    let map: HashMap<char, char> =
    vec![('0', '0'), ('1', '1'),('6', '9'),
    ('8', '8'), ('9', '6')].into_iter().collect();
    let num: Vec<char> = nums.chars().collect();
    println!("{:?}", num);
    let n = num.len();
    for i in 0..num.len() {
        if let Some(&x) = map.get(&num[n - 1 - i]) {
            if num[i] != x {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
/*
Word Pattern
Given a pattern and a string s, find if s follows the same pattern.

Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.

 

Example 1:

Input: pattern = "abba", s = "dog cat cat dog"
Output: true
Example 2:

Input: pattern = "abba", s = "dog cat cat fish"
Output: false
Example 3:

Input: pattern = "aaaa", s = "dog cat cat dog"
Output: false
Example 4:

Input: pattern = "abba", s = "dog dog dog dog"
Output: false
 

Constraints:

1 <= pattern.length <= 300
pattern contains only lower-case English letters.
1 <= s.length <= 3000
s contains only lower-case English letters and spaces ' '.
s does not contain any leading or trailing spaces.
All the words in s are separated by a single space.
*/
/////////
use std::collections::HashMap;
pub fn word_pattern(pattern: String, string: String) -> bool {
    let chars: Vec<char> = pattern.chars().collect();
    let strings: Vec<String> = string.split_whitespace().map(|s| s.to_string()).collect();
    if chars.len() != strings.len() {
        return false;
    }
    let mut hashmap1: HashMap<char, String> = HashMap::new();
    let mut hashmap2: HashMap<String, char> = HashMap::new();
    for i in 0..chars.len() {
        let c = chars[i];
        let s = strings[i].clone();
        if let Some(ss) = hashmap1.get(&c) {
            if *ss != s { return false; }
        } else {
            hashmap1.insert(c, s.clone());
        }
        if let Some(cc) = hashmap2.get(&s) {
            if *cc != c { return false; }
        } else {
            hashmap2.insert(s.clone(), c);
        }
    }
    true
}
////////
/*
Flip Game
You are playing the following Flip Game with your friend: Given a string that contains only these two characters: + and -, you and your friend take turns to flip two consecutive "++" into "--". The game ends when a person can no longer make a move and therefore the other person will be the winner.

Write a function to compute all possible states of the string after one valid move.

Example:

Input: 
s = "++++"

Output: 
[
  "--++",
  "+--+",
  "++--"
]
Note: If there is no valid move, return an empty list [].
*/
pub fn generate_possible_next_moves(s: String) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let n = s.len();
    for i in 1..n {
        if &s[i - 1..=i] == "++" {
            res.push(format!("{}{}{}", &s[0..i - 1], "--", &s[i + 1..]));
        }
    }
    res
}
//////////////
fn reverse_string(s: &mut Vec<char>) {
        s.reverse();
    }
//////////////
/*
Reverse Vowels of a String
Write a function that takes a string as input and reverse only the vowels of a string.

Example 1:

Input: "hello"
Output: "holle"
Example 2:

Input: "leetcode"
Output: "leotcede"
Note:
The vowels does not include the letter "y".
*/
pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'
    | 'A' | 'E' | 'I' | 'O' | 'U' )
}
pub fn reverse_vowels(s: String) -> String {
    let mut a: Vec<char> = s.chars().collect();
    let n = a.len();
    if n == 0 {
        return "".to_string();
    }
    let mut l = 0;
    let mut r = n - l;
    while l < r {
        if is_vowel(a[l]) && is_vowel(a[r]) {
            a.swap(l, r);
            l += 1;
            r -= 1;
        } else {
            if !is_vowel(a[l]) {
                l += 1;
            }
            if !is_vowel(a[r]) {
                r -= 1;
            }
        }
    }
    a.iter().collect()
}
//////////////
/*
Logger Rate Limiter
Design a logger system that receives a stream of messages along with their timestamps. Each unique message should only be printed at most every 10 seconds (i.e. a message printed at timestamp t will prevent other identical messages from being printed until timestamp t + 10).

All messages will come in chronological order. Several messages may arrive at the same timestamp.

Implement the Logger class:

Logger() Initializes the logger object.
bool shouldPrintMessage(int timestamp, string message) Returns true if the message should be printed in the given timestamp, otherwise returns false.
 

Example 1:

Input
["Logger", "shouldPrintMessage", "shouldPrintMessage", "shouldPrintMessage", "shouldPrintMessage", "shouldPrintMessage", "shouldPrintMessage"]
[[], [1, "foo"], [2, "bar"], [3, "foo"], [8, "bar"], [10, "foo"], [11, "foo"]]
Output
[null, true, true, false, false, false, true]

Explanation
Logger logger = new Logger();
logger.shouldPrintMessage(1, "foo");  // return true, next allowed timestamp for "foo" is 1 + 10 = 11
logger.shouldPrintMessage(2, "bar");  // return true, next allowed timestamp for "bar" is 2 + 10 = 12
logger.shouldPrintMessage(3, "foo");  // 3 < 11, return false
logger.shouldPrintMessage(8, "bar");  // 8 < 12, return false
logger.shouldPrintMessage(10, "foo"); // 10 < 11, return false
logger.shouldPrintMessage(11, "foo"); // 11 >= 11, return true, next allowed timestamp for "foo" is
                                      // 11 + 10 = 21
 

Constraints:

0 <= timestamp <= 109
Every timestamp will be passed in non-decreasing order (chronological order).
1 <= message.length <= 30
At most 104 calls will be made to shouldPrintMessage.

*/
use std::collections::HashMap;
#[derive(Default)]
pub struct Logger {
    messages: HashMap<String, i32>,
}
impl Logger {
    fn new() -> Self {
        Logger {
            messages: HashMap::new(),
        }
    }
    pub fn print_messages(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(&t) = self.messages.get(&message) {
            if timestamp < t + 10 {
                return false;
            }
        }
        self.messages.insert(message, timestamp);
        true
    }
}
/*
First Unique Character in a String
Given a string, find the first non-repeating character in it and return its index. If it doesn't exist, return -1.

Examples:

s = "leetcode"
return 0.

s = "loveleetcode"
return 2.
*/
use std::collections::HashMap;
pub fn first_unique_char(s: String) -> i32 {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let e = hm.entry(c).or_default();
        *e += 1;
    }
    for (i, c) in s.chars().enumerate() {
        if let Some(1) = hm.get(&c) {
            return i as i32;
        }
    }
    -1
}
///////////
/*
Find the Difference
You are given two strings s and t.

String t is generated by random shuffling string s and then add one more letter at a random position.

Return the letter that was added to t.

 

Example 1:

Input: s = "abcd", t = "abcde"
Output: "e"
Explanation: 'e' is the letter that was added.
Example 2:

Input: s = "", t = "y"
Output: "y"
Example 3:

Input: s = "a", t = "aa"
Output: "a"
Example 4:

Input: s = "ae", t = "aea"
Output: "a"
 

Constraints:

0 <= s.length <= 1000
t.length == s.length + 1
s and t consist of lower-case English letters.

*/
use std::collections::HashMap;
pub fn find_difference(s: String, t: String) -> char {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for c in t.chars() {
        let e = hm.entry(c).or_default();
        *e += 1;
    }
    for c in s.chars() {
        let e = hm.entry(c).or_default();
        *e -= 1;
    }
    for (&c, &v) in hm.iter() {
        if v == 1 {
            return c;
        }
    }
    unreachable!()
}
/////////////
fn main() {
    let words: Vec<String> = vec!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "practice".to_string();
    let word2 = "makes".to_string();
    println!("{:?}", shortest_distance(words, word1, word2));
}
