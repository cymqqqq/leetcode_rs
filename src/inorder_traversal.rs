/*
Binary Tree Inorder Traversal
Given the root of a binary tree, return the inorder traversal of its nodes' values.

 

Example 1:


Input: root = [1,null,2,3]
Output: [1,3,2]
Example 2:

Input: root = []
Output: []
Example 3:

Input: root = [1]
Output: [1]
Example 4:


Input: root = [1,2]
Output: [2,1]
Example 5:


Input: root = [1,null,2]
Output: [1,2]
 

Constraints:

The number of nodes in the tree is in the range [0, 100].
-100 <= Node.val <= 100
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeLink,
    pub right: TreeLink,
}
pub type TreeLink = Option<Rc<RefCell<TreeNode>>>;
#[macro_export]
macro_rules! tree {
    ($e: expr) => {
        TreeLink::leaf($e)
    };
    ($e: expr, $l: expr, $r: expr) => {
        TreeLink::branch($e, $l, $r)
    };
}
use std::cell::RefCell;
use std::rc::Rc;
pub trait TreeCreator {
    fn branch(val: i32, left: TreeLink, right: TreeLink) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right})))
    }
    fn leaf(val: i32) -> TreeLink {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}
impl TreeCreator for TreeLink {}

pub fn inorder_traver(root: TreeLink) -> Vec<i32> {
    let mut cur = root;
    let mut stack: Vec<TreeLink> = vec![];
    let mut res = vec![];
    while cur.is_some() || !stack.is_empty() {
        while let Some(node) = cur {
            let left = node.borrow_mut().left.take();
            stack.push(Some(node));
            cur = left;
        }
        let node = stack.pop().unwrap().unwrap();
        res.push(node.borrow().val);
        cur = node.borrow_mut().right.take();
    }
    res
}
fn main() {
    let x = tree!(1, None, tree!(2, tree!(3), None));
  
    println!("{:?}", inorder_traver(x));
}
