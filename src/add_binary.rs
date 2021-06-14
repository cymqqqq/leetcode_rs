/*
Add Binary
Given two binary strings a and b, return their sum as a binary string.

 

Example 1:

Input: a = "11", b = "1"
Output: "100"
Example 2:

Input: a = "1010", b = "1011"
Output: "10101"
 

Constraints:

1 <= a.length, b.length <= 104
a and b consist only of '0' or '1' characters.
Each string does not contain leading zeros except for the zero itself.
*/
pub fn add_binary(a: String, b: String) -> String {
    let x = i128::from_str_radix(&a, 2).unwrap_or(0);
    let y = i128::from_str_radix(&b, 2).unwrap_or(0);
    format!("{:b}", x + y)
}
fn main() {
    let a = String::from("1010");
    let b = String::from("1011");
    println!("{}", add_binary(a, b));
}
