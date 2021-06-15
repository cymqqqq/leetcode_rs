/*
Symmetric Tree
Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

For example, this binary tree [1,2,2,3,4,4,3] is symmetric:

    1
   / \
  2   2
 / \ / \
3  4 4  3
 

But the following [1,2,2,null,3,null,3] is not:

    1
   / \
  2   2
   \   \
   3    3
 

Follow up: Solve it both recursively and iteratively.
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
pub trait Symmetric {
    fn is_symmetric(&self) -> bool;
    fn is_mirror(&self, right: &TreeLink) -> bool;
}
impl Symmetric for TreeLink {
    fn is_symmetric(&self) -> bool {
        if let Some(node) = self {
            let node = node.borrow();
            node.left.is_mirror(&node.right)
        } else {
            true
        }
    }
    fn is_mirror(&self, right: &TreeLink) -> bool {
        match (self, right) {
            (Some(x), Some(y)) => {
                let x = x.borrow();
                let y = y.borrow();
                x.val == y.val && x.left.is_mirror(&y.right) && y.right.is_mirror(&x.left)
            }
            (None, None) => true,
            _ => false,
        }
    }
}
pub fn is_symmetric(root: TreeLink) -> bool {
    root.is_symmetric()
}
pub fn same_tree(root1: TreeLink, root2: TreeLink) -> bool {
    root1 == root2
}
fn main() {
    let x = tree!(1, None, tree!(2, tree!(3), None));
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let p = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    println!("{:?}", is_symmetric(p));
}
