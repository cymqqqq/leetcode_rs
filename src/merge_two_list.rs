/*
Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.
Input: l1 = [1,2,4], l2 = [1,3,4]
Output: [1,1,2,3,4,4]
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


pub fn merge_two_links(l1: ListLink, l2: ListLink) -> ListLink {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }
    let mut p1 = l1.unwrap();
    let mut p2 = l2.unwrap();
    if p1.val < p2.val {
        p1.next = merge_two_links(p1.next, Some(p2));
        Some(p1)
    } else {
        p2.next = merge_two_links(Some(p1), p2.next);
        Some(p2)
    }
}
fn main() {
    let a = list!(1,2,3);
    let b = list!(1,3,5);
    println!("{:?}", merge_two_links(a,b));
}
