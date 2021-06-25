/*
Excel Sheet Column Number
Given a column title as appear in an Excel sheet, return its corresponding column number.

For example:

    A -> 1
    B -> 2
    C -> 3
    ...
    Z -> 26
    AA -> 27
    AB -> 28 
    ...
Example 1:

Input: "A"
Output: 1
Example 2:

Input: "AB"
Output: 28
Example 3:

Input: "ZY"
Output: 701
 

Constraints:

1 <= s.length <= 7
s consists only of uppercase English letters.
s is between "A" and "FXSHRXW".
*/
pub fn title_num(s: String) -> i32 {
    s.bytes().fold(0, |sum, c| sum * 26 + i32::from(c) - 'A' as i32 + 1)
}
fn main() {
    println!("{:?}", title_num("AB".to_string()));
}
