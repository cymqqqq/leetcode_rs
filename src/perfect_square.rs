/*
Valid Perfect Square
Given a positive integer num, write a function which returns True if num is a perfect square else False.

Follow up: Do not use any built-in library function such as sqrt.

 

Example 1:

Input: num = 16
Output: true
Example 2:

Input: num = 14
Output: false
*/
pub fn is_perfect_square(num: i32) -> bool {
    let mut r: i64 = num as i64;
    let x: i64 = num as i64;
    while r * r > x {
        r = (r + x / r) >> 1;
        
    }
    r * r == x
}
