/*
Guess Number Higher or Lower
We are playing the Guess Game. The game is as follows:

I pick a number from 1 to n. You have to guess which number I picked.

Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.

You call a pre-defined API int guess(int num), which returns 3 possible results:

-1: The number I picked is lower than your guess (i.e. pick < num).
1: The number I picked is higher than your guess (i.e. pick > num).
0: The number I picked is equal to your guess (i.e. pick == num).
Return the number that I picked.

 

Example 1:

Input: n = 10, pick = 6
Output: 6
Example 2:

Input: n = 1, pick = 1
Output: 1
Example 3:

Input: n = 2, pick = 1
Output: 1
Example 4:

Input: n = 2, pick = 2
Output: 2
 

Constraints:

1 <= n <= 231 - 1
1 <= pick <= n
*/
static mut N: i32 = 0;
unsafe fn guess(num: i32) -> i32 {
    use std::cmp::Ordering::*;
    match N.cmp(&num) {
        Equal => 0,
        Greater => 1,
        Less => -1,
    }
}
#[allow(non_snake_case)]
unsafe fn guessnum(n: i32) -> i32 {
    let mut low = 0;
    let  high = n;
    while low < high {
        let mid = low + (high - low) >> 1;
        match guess(mid) {
            0 => { return mid; }
            -1 => { low = mid; }
            1 => { low = mid + 1;}
            _ => {}
        }
        
    }
    low
}
fn main() {
unsafe {
    println!("{:?}", guessnum(10));
}
    
}
