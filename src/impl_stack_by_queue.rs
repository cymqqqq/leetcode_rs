/*
Implement Stack using Queues
Implement a last in first out (LIFO) stack using only two queues. The implemented stack should support all the functions of a normal queue (push, top, pop, and empty).

Implement the MyStack class:

void push(int x) Pushes element x to the top of the stack.
int pop() Removes the element on the top of the stack and returns it.
int top() Returns the element on the top of the stack.
boolean empty() Returns true if the stack is empty, false otherwise.
Notes:

You must use only standard operations of a queue, which means only push to back, peek/pop from front, size, and is empty operations are valid.
Depending on your language, the queue may not be supported natively. You may simulate a queue using a list or deque (double-ended queue), as long as you use only a queue's standard operations.
 

Example 1:

Input
["MyStack", "push", "push", "top", "pop", "empty"]
[[], [1], [2], [], [], []]
Output
[null, null, null, 2, 2, false]

Explanation
MyStack myStack = new MyStack();
myStack.push(1);
myStack.push(2);
myStack.top(); // return 2
myStack.pop(); // return 2
myStack.empty(); // return False
 

Constraints:

1 <= x <= 9
At most 100 calls will be made to push, pop, top, and empty.
All the calls to pop and top are valid.

*/
use std::collections::VecDeque;
#[derive(Default)]
pub struct Mystack {
    queue: VecDeque<i32>,
}
impl Mystack {
    fn new() -> Self {
        Mystack {
            queue: VecDeque::new(),
        }
    }
    fn push(&mut self, n: i32) {
        let mut x = self.queue.len();
        self.queue.push_back(n);
        while x > 0 {
            let front = self.queue.pop_front().unwrap();
            self.queue.push_back(front);
            x -= 1;
        }
    }
    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }
    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
