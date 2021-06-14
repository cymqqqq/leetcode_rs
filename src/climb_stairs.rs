/*
Climbing Stairs
You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

 

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps
Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step
*/
pub fn climb_stairs(n: i32) -> i32 {
    match n {
        1 | 2 => n,
        k => (2..k).fold((1, 2), |acc, _| (acc.1, acc.0 + acc.1)).1,
    }
}
fn main() {

    println!("{:?}", climb_stairs(3));
}
