/*
Can Place Flowers
You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule.

 

Example 1:

Input: flowerbed = [1,0,0,0,1], n = 1
Output: true
Example 2:

Input: flowerbed = [1,0,0,0,1], n = 2
Output: false
*/
pub fn can_place_flowers(mut flower: Vec<i32>, n: i32) -> bool {
    let m = flower.len();
    let mut sum = 0;
    for i in 0..m {
        if flower[i] == 0 {
            if i == 0 {
                sum += 1;
                flower[i] = 1;
            } else {
                if flower[i - 1] == 0 {
                    sum += 1;
                    flower[i] = 1;
                }
            }
        } else {
            if i > 0 && flower[i - 1] == 1 {
                sum -= 1;
            }
        }
    }
    sum >= n
}
