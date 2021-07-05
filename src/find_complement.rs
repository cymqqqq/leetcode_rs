/*
Number Complement
Given a positive integer num, output its complement number. The complement strategy is to flip the bits of its binary representation.

 

Example 1:

Input: num = 5
Output: 2
Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
Example 2:

Input: num = 1
Output: 0
Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
*/
pub fn find_complement(num: i32) -> i32 {
    let mut mask = !0;
    println!("mask {:?}", mask);
    while mask & num != 0 {
        println!("mask & num {:?}", mask & num);
        mask <<= 1;
        println!("mask <<= 1 = {:?}", mask);
    }
    !mask & !num
}
fn main() {
    println!("{:?}", find_complement(5));
}
