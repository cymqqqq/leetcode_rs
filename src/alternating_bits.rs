/*
Binary Number with Alternating Bits
Given a positive integer, check whether it has alternating bits: namely, if two adjacent bits will always have different values.

 

Example 1:

Input: n = 5
Output: true
Explanation: The binary representation of 5 is: 101
Example 2:

Input: n = 7
Output: false
Explanation: The binary representation of 7 is: 111.
Example 3:

Input: n = 11
Output: false
Explanation: The binary representation of 11 is: 1011.
Example 4:

Input: n = 10
Output: true
Explanation: The binary representation of 10 is: 1010.
*/
pub fn has_alternating_bits(n: i32) -> bool {
    let a = (n >> 1) ^ n;
    println!("{:?}", a);
    println!("{:?}", (a + 1) & a);
    (a + 1) & a == 0
}
fn main() {
    println!("{:?}", has_alternating_bits(5));
}
