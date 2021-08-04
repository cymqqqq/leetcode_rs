/*
Kth Largest Element in a Stream
Design a class to find the kth largest element in a stream. Note that it is the kth largest element in the sorted order, not the kth distinct element.

Implement KthLargest class:

KthLargest(int k, int[] nums) Initializes the object with the integer k and the stream of integers nums.
int add(int val) Returns the element representing the kth largest element in the stream.
 

Example 1:

Input
["KthLargest", "add", "add", "add", "add", "add"]
[[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
Output
[null, 4, 5, 5, 8, 8]

Explanation
KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
kthLargest.add(3);   // return 4
kthLargest.add(5);   // return 5
kthLargest.add(10);  // return 5
kthLargest.add(9);   // return 8
kthLargest.add(4);   // return 8

*/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Kthlargest {
    pq: BinaryHeap<Reverse<i32>>,
    k: usize,
}
impl Kthlargest {
    fn new(k: i32, num: Vec<i32>) -> Self {
        let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let k = k as usize; 
        for i in num {
            pq.push(Reverse(i));
            if pq.len() > k {
                pq.pop();
            }
        }
        Kthlargest { pq, k }
    }
    fn add(&mut self, val: i32) -> i32 {
        self.pq.push(Reverse(val));
        if self.pq.len() > self.k {
            self.pq.pop();
        }
        let x = *self.pq.peek().unwrap();
        x.0
    }
}
