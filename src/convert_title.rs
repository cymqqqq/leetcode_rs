/*
Excel Sheet Column Title
Given a positive integer, return its corresponding column title as appear in an Excel sheet.

For example:

    1 -> A
    2 -> B
    3 -> C
    ...
    26 -> Z
    27 -> AA
    28 -> AB 
    ...
Example 1:

Input: 1
Output: "A"
Example 2:

Input: 28
Output: "AB"
Example 3:

Input: 701
Output: "ZY"
*/
pub fn convert_title(mut n: i32) -> String {
    let mut v: Vec<char> = vec![];
    while n > 0 {
        let x = ((n - 1) % 26) as u8;
        let c = (x + b'A') as char;
        v.insert(0, c);
        n = (n - 1) / 26;
    }
    v.iter().collect()
}
fn main() {
    let  i = 203;
    println!("{:?}", convert_title(i));
}
