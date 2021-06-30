/*
Strobogrammatic Number
A strobogrammatic number is a number that looks the same when rotated 180 degrees (looked at upside down).

Write a function to determine if a number is strobogrammatic. The number is represented as a string.

 

Example 1:

Input: num = "69"
Output: true
Example 2:

Input: num = "88"
Output: true
Example 3:

Input: num = "962"
Output: false
Example 4:

Input: num = "1"
Output: true

*/
use std::collections::HashMap;
pub fn is_strobogrammatic(nums: String) -> bool {
    let map: HashMap<char, char> =
    vec![('0', '0'), ('1', '1'),('6', '9'),
    ('8', '8'), ('9', '6')].into_iter().collect();
    let num: Vec<char> = nums.chars().collect();
    println!("{:?}", num);
    let n = num.len();
    for i in 0..num.len() {
        if let Some(&x) = map.get(&num[n - 1 - i]) {
            if num[i] != x {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
fn main() {

    
    println!("{:?}", is_strobogrammatic("963".to_string()));
}
