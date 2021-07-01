/*
Power of Two
Given an integer n, return true if it is a power of two. Otherwise, return false.

An integer n is a power of two, if there exists an integer x such that n == 2x.

 

Example 1:

Input: n = 1
Output: true
Explanation: 20 = 1
Example 2:

Input: n = 16
Output: true
Explanation: 24 = 16
Example 3:

Input: n = 3
Output: false
Example 4:

Input: n = 4
Output: true
Example 5:

Input: n = 5
Output: false
 

Constraints:

-231 <= n <= 231 - 1
*/
pub fn power_of_two(n: i32) -> bool {
    if n <= 0 { return false; }
    n & (n - 1) == 0
}
fn main() {
    let num = 20;
    println!("{:?}", power_of_two(num));
}
/*
Power of Four
Given an integer n, return true if it is a power of four. Otherwise, return false.

An integer n is a power of four, if there exists an integer x such that n == 4x.

 

Example 1:

Input: n = 16
Output: true
Example 2:

Input: n = 5
Output: false
Example 3:

Input: n = 1
Output: true
 

Constraints:

-231 <= n <= 231 - 1
               */
fn is_power_of_four(mut n: i32) -> bool {
        while n != 0 && n % 4 == 0 {
            n /= 4;
        }
        n == 1
    }
