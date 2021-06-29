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
/////////////////////
//invert binary tree
pub fn invert_bt(root: TreeLink) -> TreeLink {
    if let Some(node) = &root {
        let mut node = node.borrow_mut();
        let left = node.left.take();
        let right = node.right.take();
        node.right = invert_bt(left);
        node.left = invert_bt(right);
    }
    root
}
////////////////////
///lower common ancestor for binary tree
pub fn lowest_common_ancestor(mut root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
    let p_val = p.unwrap().borrow().val;
    let q_val = q.unwrap().borrow().val;
    while let Some(node) = root.clone() {
        let mut node = node.borrow_mut();
        let val = node.val;
        if val > p_val && val > q_val {
            root = node.left.take();
            continue;
        }
        if val < p_val && val < q_val {
            root = node.right.take();
            continue;
        }
        node.left.take();
        node.right.take();
        break;
    }
    root
}
////////////////////
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
////////////
pub fn same_tree(root1: TreeLink, root2: TreeLink) -> bool {
    root1 == root2
}
//////////////
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
///////////////////
trait Maxdepth {
    fn max_depth(&self) -> i32;
}
impl Maxdepth for TreeLink {
    fn max_depth(&self) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            return 1 + i32::max(node.left.max_depth(), node.right.max_depth());
        }
        0
    }
}
pub fn max_depth(root: TreeLink) -> i32 {
    root.max_depth()
}
///////////////////
pub fn sorted_array_to_bst(nums: Vec<i32>) -> TreeLink {
    let n = nums.len();
    match n {
        0 => None,
        1 => tree!(nums[0]),
        _ => {
            let mid = n >> 1;
            let left = nums[0..mid].to_vec();
            let right = nums[mid + 1..].to_vec();
            tree! (
                nums[mid],
                sorted_array_to_bst(left),
                sorted_array_to_bst(right)
            )
        }
    }
}
///////////////////
trait Height {
    fn height(&self) -> usize;
}
impl Height for TreeLink {
    fn height(&self) -> usize {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                usize::max(node.left.height(), node.right.height() + 1)
            }
        }
    }
}
trait Balanced {
    fn is_balanced(&self) -> bool;
}
impl Balanced for TreeLink {
    fn is_balanced(&self) -> bool {
        match self {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let left = &node.left;
                let right = &node.right;
                let height_left = left.height();
                let height_right = right.height();
                if height_left == height_right
                    || height_left == height_right + 1
                    || height_left + 1 == height_right
                {
                    left.is_balanced() && right.is_balanced()
                } else {
                    false
                }
            }
        }
    }
}
pub fn is_balanced(root: TreeLink) -> bool {
    root.is_balanced()
}
///////////////////
trait Mindepth {
    fn min_depth(&self) -> usize;
}
impl Mindepth for TreeLink {
    fn min_depth(&self) -> usize {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (&node.left, &node.right) {
                    (None, None) => 1,
                    (Some(_), None) => TreeLink::min_depth(&node.left) + 1,
                    (None, Some(_)) => TreeLink::min_depth(&node.right) + 1,
                    (Some(_), Some(_)) => {
                        usize::min(
                            TreeLink::min_depth(&node.left),
                            TreeLink::min_depth(&node.right),
                        ) + 1
                    }
                }
            }
        }
    }
}
pub fn min_depth(root: TreeLink) -> i32 {
    root.min_depth() as i32
}
///////////////////
trait Pathsum {
    fn path_sum(&self, sum: i32) -> bool;
}
impl Pathsum for TreeLink {
    fn path_sum(&self, sum: i32) -> bool {
        if let Some(node) = self {
            let node = node.borrow();
            let val = node.val;
            let left = &node.left;
            let right = &node.right;
            if left.is_some() && right.is_some() { sum == val }
            else { right.path_sum(sum - val) || left.path_sum(sum - val) }
            
        } else { false }
    }
}
pub fn path_sum(root: TreeLink, sum: i32) -> bool {
    root.path_sum(sum)
}
///////////////////
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
///////////////////
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
    let nums = vec![-10, -3, 0, 5, 9];
    println!("{:?}", path_sum(q, 3));
}
