/*
Remove Duplicates from Sorted List
Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

 

Example 1:


Input: head = [1,1,2]
Output: [1,2]
Example 2:


Input: head = [1,1,2,3,3]
Output: [1,2,3]
*/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}
pub type ListLink = Option<Box<ListNode>>;
pub trait ListCreator {
    fn create(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode {val, next}))
    }
}
impl ListCreator for ListLink {}
#[macro_export]
macro_rules! list {
    () => { None; };
    ($e: expr) => { ListLink::create($e, None) };
    ($e: expr, $($tail: tt)*) => { ListLink::create($e, list!($($tail)*)) };
}
pub fn delete_duplication(mut head: ListLink) -> ListLink {
    let mut p = head.as_mut();
    while let Some(n) = p {
        while let Some(m) = n.next.as_mut() {
            if m.val != n.val {
                break;
            }
            n.next = m.next.take();
        }
        p = n.next.as_mut();
    }
    head
}
fn main() {
    let x = list!(1,1,2,3,3);
    println!("{:?}", delete_duplication(x));
}
