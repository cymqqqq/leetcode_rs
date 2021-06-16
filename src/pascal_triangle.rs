/*
Pascal's Triangle
Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.


In Pascal's triangle, each number is the sum of the two numbers directly above it.

Example:

Input: 5
Output:
[
     [1],
    [1,1],
   [1,2,1],
  [1,3,3,1],
 [1,4,6,4,1]
]
*/
pub fn generate(num_row: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    for i in 0..num_row {
        let ui = i as usize;
        res.push(vec![]);
        for j in 0..=i {
            let uj = j as usize;
            if j == 0 || i == j {
                res[ui].push(1);
            } else {
                let prev = &res[ui - 1];
                let sum = prev[uj - 1] + prev[uj];
                res[ui].push(sum)
            }
        }
    }
    res
}
fn main() {
    println!("{:?}", generate(5));
}
