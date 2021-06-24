/*
Two Sum III - Data structure design
Design a data structure that accepts a stream of integers and checks if it has a pair of integers that sum up to a particular value.

Implement the TwoSum class:

TwoSum() Initializes the TwoSum object, with an empty array initially.
void add(int number) Adds number to the data structure.
boolean find(int value) Returns true if there exists any pair of numbers whose sum is equal to value, otherwise, it returns false.
 

Example 1:

Input
["TwoSum", "add", "add", "add", "find", "find"]
[[], [1], [3], [5], [4], [7]]
Output
[null, null, null, null, true, false]

Explanation
TwoSum twoSum = new TwoSum();
twoSum.add(1);   // [] --> [1]
twoSum.add(3);   // [1] --> [1,3]
twoSum.add(5);   // [1,3] --> [1,3,5]
twoSum.find(4);  // 1 + 3 = 4, return true
twoSum.find(7);  // No two integers sum up to 7, return false
 

Constraints:

-105 <= number <= 105
-231 <= value <= 231 - 1
At most 5 * 104 calls will be made to add and find.
*/
use std::collections::HashMap;
use std::i32;
#[derive(Default)]
pub struct Twosum {
    nums: HashMap<i32, usize>,
    max: i32,
    min: i32,
}
impl Twosum {
    fn new() -> Self {
        Twosum {
            nums: HashMap::new(),
            max: i32::MIN,
            min: i32::MAX,
        }
    }
    fn add(&mut self, num: i32) {
        self.max = i32::max(self.max, num << 1);
        self.min = i32::min(self.min, num << 1);
        self.nums.insert(num, self.nums.get(&num)
        .unwrap_or(&0) + 1);
    }
    fn find(&self, val: i32) -> bool {
        if val < self.min || val > self.max {
            return false;
        }
        for (&i, &v) in &self.nums {
            let b = val - i;
            if i == b && v == 2 {
                return true;
            }
            if i != b && self.nums.contains_key(&b) {
                return true;
            }
        }
        false
    }
}
fn main() {
    let mut num = Twosum::new();
    num.add(1);
    num.add(3);
    num.add(2);
    println!("{:?}", num.find(0));
}
