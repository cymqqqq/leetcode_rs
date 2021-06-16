/*
Binary Tree Preorder Traversal
Given the root of a binary tree, return the preorder traversal of its nodes' values.

 

Example 1:


Input: root = [1,null,2,3]
Output: [1,2,3]
Example 2:

Input: root = []
Output: []
Example 3:

Input: root = [1]
Output: [1]
Example 4:


Input: root = [1,2]
Output: [1,2]
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
        TreeLink::branch($e,$l,$r)
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
            left:None,
            right:None,
        })))
    }
}
impl TreeCreator for TreeLink {}
trait Preorder {
    fn preorder(&self, nums: &mut Vec<i32>);
}
impl Preorder for TreeLink {
    fn preorder(&self, nums: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            nums.push(node.val);
            node.left.preorder(nums);
            node.right.preorder(nums);
        }
    }
}
pub fn preorder_traver(root: TreeLink) -> Vec<i32> {
    let mut res = vec![];
    root.preorder(&mut res);
    res
}

fn main() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    println!("{:?}", preorder_traver(root));
}
