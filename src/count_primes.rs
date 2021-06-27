/*
Count Primes
Count the number of prime numbers less than a non-negative number, n.

 

Example 1:

Input: n = 10
Output: 4
Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
Example 2:

Input: n = 0
Output: 0
Example 3:

Input: n = 1
Output: 0
 

Constraints:

0 <= n <= 5 * 106
*/
pub fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }
    let x = n as usize;
    let mut a: Vec<bool> = vec![true; x];
    a[0] = false;
    a[1] = false;
    let mut i: usize = 2;
    while i * i < x {
        if a[i] {
            let mut j = 2;
            while j * i < x {
                a[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    let sum: i32 = a.iter().fold(0, |sum, &b| if b {sum + 1} else {sum});
    sum
}
fn main() {
    let n = 10;
    println!("{:?}", count_primes(n));
}
