/*
Count Binary Substrings
Give a string s, count the number of non-empty (contiguous) substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.

Substrings that occur multiple times are counted the number of times they occur.

Example 1:
Input: "00110011"
Output: 6
Explanation: There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".

Notice that some of these substrings repeat and are counted the number of times they occur.

Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.

*/
pub fn count_binary_substrings(s: String) -> i32 {
    let mut prev: usize = 0;
    let mut cur: usize = 0;
    let mut cur_c: Option<char> = None;
    let mut res = 0;
    for c in s.chars() {
        println!("c = {:?}", c);
        if let Some(cc) = cur_c {
            println!("cc = {:?}, c = {:?}", cc, c);
            if cc == c { cur += 1; }
            else {
                res += usize::min(prev, cur);
                println!("res = {:?}", res);
                cur_c = Some(c);
                println!("cur_c = {:?}", cur_c);
                prev = cur;
                println!("prev = {:?}", prev);
                cur = 1;
            }
        } else {
            cur_c = Some(c);
            println!("cur_c = {:?}", cur_c);
            cur = 1;
        }
    }
    res += usize::min(prev, cur);
    res as i32
}
fn main() {
    println!("{:?}", count_binary_substrings("1010".to_string()));
}
