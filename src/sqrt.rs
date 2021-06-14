/*
Sqrt(x)
Given a non-negative integer x, compute and return the square root of x.

Since the return type is an integer, the decimal digits are truncated, and only the integer part of the result is returned.

 

Example 1:

Input: x = 4
Output: 2
Example 2:

Input: x = 8
Output: 2
*/
pub fn sqrt_(x: i32) -> i32 {
    (x as f64).sqrt().floor() as i32
}
fn main() {

    println!("{:?}", sqrt_(8));
}
