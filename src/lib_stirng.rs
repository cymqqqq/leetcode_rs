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
/*
Is Subsequence
Given a string s and a string t, check if s is subsequence of t.

A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).

Follow up:
If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?

Credits:
Special thanks to @pbrother for adding this problem and creating all test cases.

 

Example 1:

Input: s = "abc", t = "ahbgdc"
Output: true
Example 2:

Input: s = "axc", t = "ahbgdc"
Output: false
 

Constraints:

0 <= s.length <= 100
0 <= t.length <= 10^4
Both strings consists only of lowercase characters.
*/
pub fn subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    while i < s.len() && j < t.len() {
        if s[i..=i] == t[j..=j] {
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }
    i == s.len()
}
/////////////
pub fn read_binary(num: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for i in 0..11 {
        for j in 0..60 {
            if i32::count_ones(i) + i32::count_ones(j) == num as u32 {
                res.push(format!("{}: {:02}", i , j));
            }
        }
    }
    res
}
/*
Valid Word Abbreviation
Given a non-empty string s and an abbreviation abbr, return whether the string matches with the given abbreviation.

A string such as "word" contains only the following valid abbreviations:

["word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1", "w1r1", "1o2", "2r1", "3d", "w3", "4"]
Notice that only the above abbreviations are valid abbreviations of the string "word". Any other string is not a valid abbreviation of "word".

Note:
Assume s contains only lowercase letters and abbr contains only lowercase letters and digits.

Example 1:
Given s = "internationalization", abbr = "i12iz4n":

Return true.
Example 2:
Given s = "apple", abbr = "a2e":

Return false.
*/
pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    let n = word.len();
    let m = abbr.len();
    let word: Vec<char> = word.chars().collect();
    let abbr: Vec<char> = abbr.chars().collect();
    while i < n && j < m {
        if abbr[j].is_alphabetic() {
            if abbr[j] != word[i] {
                return false;
            }
            j += 1;
            i += 1;
        } else {
            let mut w = 0;
            while j < m && abbr[j].is_numeric() {
                if w == 0 && abbr[j] == '0' {
                    return false;
                }
                w *= 10;
                w += (abbr[j] as u8 - b'0') as usize;
                j += 1;
            }
            i += w;
        }
    }
    i == n && j == m
}
/*
Longest Palindrome
Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.

Letters are case sensitive, for example, "Aa" is not considered a palindrome here.

 

Example 1:

Input: s = "abccccdd"
Output: 7
Explanation:
One longest palindrome that can be built is "dccaccd", whose length is 7.
Example 2:

Input: s = "a"
Output: 1
Example 3:

Input: s = "bb"
Output: 2
*/
use std::collections::HashSet;
pub fn longest_palindrome(s: String) -> i32 {
    let mut hs: HashSet<char> = HashSet::new();
    let mut mid = 0;
    for c in s.chars() {
        if hs.contains(&c) {
            hs.remove(&c);
            mid += 1;
        } else {
            hs.insert(c);
        }
    }
    if hs.is_empty() {
        2 * mid
    } else {
        2 * mid + 1
    }
}
/*
Add Strings
Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.

Note:

The length of both num1 and num2 is < 5100.
Both num1 and num2 contains only digits 0-9.
Both num1 and num2 does not contain any leading zero.
You must not use any built-in BigInteger library or convert the inputs to integer directly.

*/
pub fn add_strings(str1: String, str2: String) -> String {
    let s1: Vec<i32> = str1.bytes().map(|x| (x - b'0') as i32).rev().collect();
    println!("{:?}", s1);
    let s2: Vec<i32> = str2.bytes().map(|x| (x - b'0') as i32).rev().collect();
    let carry = 0;
    let mut i = 0;
    let mut str3: Vec<char> = vec![];
    while i < s1.len() || i < s2.len() || carry > 0 {
        let mut v = 0;
        if i < s1.len() {
            v += s1[i];
        }
        if i < s2.len() {
            v += s2[i];
        }
        v += carry;
        str3.push(((v % 10) as u8 + b'0') as char);
        i += 1;
    }
    let res: String = str3.iter().rev().collect();
    res
}
/*
Number of Segments in a String
You are given a string s, return the number of segments in the string. 

A segment is defined to be a contiguous sequence of non-space characters.

 

Example 1:

Input: s = "Hello, my name is John"
Output: 5
Explanation: The five segments are ["Hello,", "my", "name", "is", "John"]
Example 2:

Input: s = "Hello"
Output: 1
Example 3:

Input: s = "love live! mu'sic forever"
Output: 4
Example 4:

Input: s = ""
Output: 0
*/
pub fn count_segments(s: String) -> i32 {
    s.split_whitespace().count() as i32
}
/*
Reverse String II
Given a string and an integer k, you need to reverse the first k characters for every 2k characters counting from the start of the string. If there are less than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and left the other as original.
Example:
Input: s = "abcdefg", k = 2
Output: "bacdfeg"

*/
pub fn rev_k(s: &mut [char], k: usize) -> &[char] {
    if s.len() <= k {
        s.reverse();
    } else {
        let half = &mut s[0..k];
        half.reverse();
    }
    s
}
pub fn reverse_str(s: String, k: i32) -> String {
    let k: usize = k as usize;
    let mut s: Vec<char> = s.chars().collect();
    let n = s.len();
    let mut i = 0;
    while i * 2 * k < n {
        let r = (i + 1) * 2 * k;
        if r < n {
            let ss = &mut s[i * 2 * k..r];
            rev_k(ss, k);
        } else {
            let ss = &mut s[i * 2 * k..n];
            rev_k(ss, k);
        }
        i += 1;
    }
    s.iter().collect()
}
/*
Reverse Words in a String III
Given a string, you need to reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

Example 1:
Input: "Let's take LeetCode contest"
Output: "s'teL ekat edoCteeL tsetnoc"
*/
pub fn reverse_words(s: String) -> String {
    let words: Vec<String> = s
    .split_whitespace()
    .map(|s| s.chars().rev().collect())
    .collect();
    let res: String = words.join(" ");
    res
}
/*
Design Compressed String Iterator
Design and implement a data structure for a compressed string iterator. The given compressed string will be in the form of each letter followed by a positive integer representing the number of this letter existing in the original uncompressed string.

Implement the StringIterator class:

next() Returns the next character if the original string still has uncompressed characters, otherwise returns a white space.
hasNext() Returns true if there is any letter needs to be uncompressed in the original string, otherwise returns false.
 

Example 1:

Input
["StringIterator", "next", "next", "next", "next", "next", "next", "hasNext", "next", "hasNext"]
[["L1e2t1C1o1d1e1"], [], [], [], [], [], [], [], [], []]
Output
[null, "L", "e", "e", "t", "C", "o", true, "d", true]

Explanation
StringIterator stringIterator = new StringIterator("L1e2t1C1o1d1e1");
stringIterator.next(); // return "L"
stringIterator.next(); // return "e"
stringIterator.next(); // return "e"
stringIterator.next(); // return "t"
stringIterator.next(); // return "C"
stringIterator.next(); // return "o"
stringIterator.hasNext(); // return True
stringIterator.next(); // return "d"
stringIterator.hasNext(); // return True

*/
pub struct Stringiterator {
    idx: usize,
    pairs: Vec<Pair>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair {
    c: char,
    m: usize,
}
impl Pair {
    fn new(c: char, m: usize) -> Self {
        Pair { c, m }
    }
}
impl Stringiterator {
    fn new(compressed_string: String) -> Self {
        let s: Vec<char> = compressed_string.chars().collect();
        let n = s.len();
        let mut i = 0;
        let mut pairs: Vec<Pair> = vec![];
        while i < n {
            let c = s[i];
            i += 1;
            let mut m: usize = 0;
            while i < n && s[i].is_numeric() {
                m *= 10;
                m += (s[i] as u8 - b'0') as usize;
                i += 1;
            }
            pairs.push(Pair::new(c, m));
        }
        Stringiterator { idx: 0, pairs }
    }
    fn next(&mut self) -> char {
        if self.idx == self.pairs.len() {
            ' '
        } else {
            let c = self.pairs[self.idx].c;
            self.pairs[self.idx].m -= 1;
            if self.pairs[self.idx].m == 0 {
                self.idx += 1;
            }
            c
        }
    }
    fn has_next(&self) -> bool {
        self.idx < self.pairs.len()
    }
}
/////////////
fn main() {
    let words: Vec<String> = vec!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "practice".to_string();
    let word2 = "makes".to_string();
    println!("{:?}", shortest_distance(words, word1, word2));
}
