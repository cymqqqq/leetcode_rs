/*
Hamming Distance
The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given two integers x and y, calculate the Hamming distance.

Note:
0 â‰¤ x, y < 231.

Example:

Input: x = 1, y = 4

Output: 2

Explanation:
1   (0 0 0 1)
4   (0 1 0 0)
       â†‘   â†‘

The above arrows point to positions where the corresponding bits are different.
*/
pub fn hamming_distance(x: i32, y: i32) -> i32 {
    let mut z = x ^ y;
    println!("{:?}", z);
    let mut sum = 0;
    for _ in 0..32 {
        sum += z & 1;
        println!("sum {:?}", sum); 
        z >>= 1;
        println!("z {:?}", z);
    }
    sum
}
fn main() {
    println!("{:?}", hamming_distance(1, 4));
}
