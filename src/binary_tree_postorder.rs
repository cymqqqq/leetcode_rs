/*
Binary Tree Postorder Traversal
Given the root of a binary tree, return the postorder traversal of its nodes' values.

 

Example 1:


Input: root = [1,null,2,3]
Output: [3,2,1]
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
Output: [2,1]
 

Constraints:

The number of the nodes in the tree is in the range [0, 100].
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
trait Postorder {
    fn postorder(&self, nums: &mut Vec<i32>);
}
impl Postorder for TreeLink {
    fn postorder(&self, nums: &mut Vec<i32>) {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.postorder(nums);
            node.right.postorder(nums);
            nums.push(node.val);
        }
    }
}
pub fn postorder(root: TreeLink) -> Vec<i32> {
    let mut res = vec![];
    root.postorder(&mut res);
    res
}
fn main() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    println!("{:?}", postorder(root));
}
