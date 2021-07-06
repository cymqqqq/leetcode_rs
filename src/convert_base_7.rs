/*
Base 7
Given an integer, return its base 7 string representation.

Example 1:
Input: 100
Output: "202"
Example 2:
Input: -7
Output: "-10"
Note: The input will be in range of [-1e7, 1e7].
*/
pub fn convert_to_base7(mut num: i32) -> String {
    if num == 0 { return "0".to_string(); }
    let mut base_7: Vec<char> = vec![];
    let minus: bool = num < 0;
    if num < 0 { num = -num; }
    while num > 0 {
        let c = ((num % 7) as u8 + b'0') as char;
        base_7.push(c);
        num /= 7;
    }
    if minus { base_7.push('_'); }
    base_7.reverse();
    let res: String = base_7.iter().collect();
    res
}
fn main() {
    println!("{:?}", convert_to_base7(100));
}
