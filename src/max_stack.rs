use std::i32;
#[derive(Default)]
pub struct MaxStack {
    vec: Vec<i32>,
}
impl MaxStack {
    fn new() -> Self {
        MaxStack { vec: vec![] }
    }
    fn push(&mut self, n: i32) {
        self.vec.push(n);
    }
    fn pop(&mut self) -> i32 {
        self.vec.pop().unwrap()
    }
    fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }
    fn peek_max(&self) -> i32 {
        let mut max = self.top();
        for &i in &self.vec {
            max = i32::max(max, i);
        }
        max
    }
    fn pop_max(&mut self) -> i32 {
        let mut idx = self.vec.len() - 1;
        let mut max = self.top();
        let n = self.vec.len();
        for i in 0..n {
            let j = n - i - 1;
            let x = self.vec[j];
            if x > max {
                max = x;
                idx = j;
            }
        }
        self.vec.remove(idx);
        max
    }
}
