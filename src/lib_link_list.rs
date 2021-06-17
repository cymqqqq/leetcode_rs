#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListLink,
}
#[macro_export]
macro_rules! list {
    () => {
        None
    };
    ($e: expr) => {
        ListLink::link($e, None)
    };
    ($e: expr, $($tail: tt)*) => {
        ListLink::link($e, list!($($tail)*))
    };
}
pub type ListLink = Option<Box<ListNode>>;
pub trait ListCreator {
    fn link(val: i32, next: ListLink) -> ListLink {
        Some(Box::new(ListNode {val, next}))
    }
}
impl ListCreator for ListLink {}
/////////merge two linklist
pub fn merge_lists(l1: ListLink, l2: ListLink) -> ListLink {
    if l1.is_some() {
        return l2;
    }
    if l2.is_some() {
        return l1;
    }
    let mut p1 = l1.unwrap();
    let mut p2 = l2.unwrap();
    if p1.val < p2.val {
        p1.next = merge_lists(p1.next, Some(p2));
        Some(p1)
    } else {
        p2.next = merge_lists(Some(p1), p2.next);
        Some(p2)
    }
    
}
/////////remvoe dup from list
pub fn delete_dup(mut head: ListLink) -> ListLink {
    let mut p = head.as_mut();
    while let Some(n) = p {
        while let Some(m) = n.next.as_mut() {
            if m.val != n.val {
                break;
            }
            n.next = m.next.take();
        }
        p = n.next.take();
    }
    head
}
/////////remove linklist element
pub fn remove_ele(mut head: ListLink, val: i32) -> ListLink {
    let mut p = &mut head;
    while p.is_some() {
        if p.as_ref().unwrap().val == val {
            *p = p.take().unwrap().next.take();
        } else {
            p = &mut p.as_mut().unwrap().next;
        }
    }
    head
}
////////reverse linklist
pub fn reverse_list(head: ListLink) -> ListLink {
    let mut p = head;
    let mut prev = None;
    while let Some(mut node) = p {
        p = node.next;
        node.next = prev;
        prev = Some(node)
    }
    prev
}
////////parlimdrome linklist
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct List {
    head: ListLink,
}
impl List {
    fn new(head: ListLink) -> Self {
        List { head }
    }
    fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }
    fn into_iter(self) -> IntoIter {
        IntoIter {  list: self }
    }
}
pub struct IntoIter {
    list: List,
}
impl Iterator for IntoIter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}
pub fn list_is_palindrome(head: ListLink) -> bool {
    let list = List::new(head);
    let vec: Vec<i32> = list.into_iter().collect();
    for (i, &v) in vec.iter().rev().enumerate() {
        if v != vec[i] {
            return false;
        }
    }
    true
}
