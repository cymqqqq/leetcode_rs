/*
Self Dividing Numbers
A self-dividing number is a number that is divisible by every digit it contains.

For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.

Also, a self-dividing number is not allowed to contain the digit zero.

Given a lower and upper number bound, output a list of every possible self dividing number, including the bounds if possible.

Example 1:
Input: 
left = 1, right = 22
Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
*/
pub fn is_self_divding(x: i32) -> bool {
    let mut a = x;
    while a > 0 {
        let last = a % 10;
        if last == 0  { return false; }
        else {
            if a % last != 0 {
                return false;
            }
            a /= 10;
        }
    }
    true
}
pub fn self_divding_num(left: i32, right: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for i in left..=right {
        if is_self_divding(i) {
            res.push(i);
        }
    }
    res
}
