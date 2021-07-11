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
/*
Binary Tree Paths
Given a binary tree, return all root-to-leaf paths.

Note: A leaf is a node with no children.

Example:

Input:

   1
 /   \
2     3
 \
  5

Output: ["1->2->5", "1->3"]

Explanation: All root-to-leaf paths are: 1->2->5, 1->3
*/
pub struct Path {
    stack: Vec<i32>,
}
impl ToString for Path {
    fn to_string(&self) -> String {
        let s: Vec<String> = self.stack.iter().map(|i| i.to_string()).collect();
        s.join("->")
    }
}
pub trait Preorder {
    fn preorder(&self, path: &mut Path, v: &mut Vec<String>);
}
impl Preorder for TreeLink {
    fn preorder(&self, path: &mut Path, v: &mut Vec<String>) {
        if let Some(node) = self {
            let node = node.borrow();
            path.stack.push(node.val);
            if node.left.is_none() && node.right.is_none() {
                v.push(path.to_string());
            }
            if node.left.is_none() {
                node.left.preorder(path, v);
            }
            if node.right.is_none() {
                node.right.preorder(path, v);
            }
            path.stack.pop();
        }
    }
}
pub fn binary_tree_path(root: TreeLink) -> Vec<String> {
    let mut path = Path { stack: vec![] };
    let mut res = vec![];
    root.preorder(&mut path, &mut res);
    root
}
////////////////
/*
Closest Binary Search Tree Value
Given a non-empty binary search tree and a target value, find the value in the BST that is closest to the target.

Note:

Given target value is a floating point.
You are guaranteed to have only one unique value in the BST that is closest to the target.
Example:

Input: root = [4,2,5,1,3], target = 3.714286

    4
   / \
  2   5
 / \
1   3

Output: 4
*/
pub trait Closet {
    fn search(&self, target: f64) -> i32;
    fn preorder(link: &TreeLink, diff: &mut f64, res: &mut i32, target: f64);
}
impl Closet for TreeLink {
    fn search(&self, target: f64) -> i32 {
        let mut diff = std::f64::MAX;
        let mut res = 0;
        preorder(&self, &mut diff, &mut res, target);
        res
    }
    fn preorder(link: &TreeLink, diff: &mut f64, res: &mut f32, target: f64) {
        if let Some(node) = link {
            let node = node.borrow();
            let val = node.val as f64;
            let delta = (val - target).abs();
            if delta < *diff {
                *diff = delta;
                *res = node.val;
            }
            if target < val {
                preorder(&node.left, diff, res, target);
            }
            if target > val {
                preorder(&node.right, diff, res, target);
            }
        }
    }
}
pub fn closet_value(root: TreeLink, target: f64) -> i32 {
    root.search(target)
}
/*
Sum of Left Leaves
Find the sum of all left leaves in a given binary tree.

Example:

    3
   / \
  9  20
    /  \
   15   7

There are two left leaves in the binary tree, with values 9 and 15 respec
*/
pub trait Sumofleftleaves {
    fn sum_of_left_leaves(&self) -> i32;
}
impl Sumofleftleaves for TreeLink {
    fn sum_of_left_leaves(&self) -> i32 {
        let mut sum = 0;
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if let Some(left_node) = left {
                let left_node = left_node.borrow();
                if left_node.left.is_none() && left_node.right.is_none() {
                    sum += left_node.val;
                } else {
                    sum += sum_of_left_leaves(left);
                }
            }
            sum += sum_of_left_leaves(right);
        }
        sum
    }
}
pub fn sum_left(root: TreeLink) -> i32 {
    root.sum_of_left_leaves()
}
/*
Find Mode in Binary Search Tree
Given a binary search tree (BST) with duplicates, find all the mode(s) (the most frequently occurred element) in the given BST.

Assume a BST is defined as follows:

The left subtree of a node contains only nodes with keys less than or equal to the node's key.
The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
Both the left and right subtrees must also be binary search trees.
 

For example:
Given BST [1,null,2,2],

   1
    \
     2
    /
   2
 

return [2].

Note: If a tree has more than one mode, you can return them in any order.

Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).
*/
pub trait Inorder {
    fn inorder(&self, prev: &mut Option<i32>, count: &mut usize, f: &mut impl FnMut(i32, usize));
}
impl Inorder for TreeLink {
    fn inorder(&self, prev: &mut Option<i32>, count: &mut usize, f: &mut impl  FnMut(i32, usize)) {
        if let Some(node) = self {
            let node = node.borrow();
            inorder(&node.left, prev, count, f);
            if let Some(prev_val) = prev.as_mut() {
                if *prev_val == node.val {
                    *count += 1;
                } else {
                    *count += 1;
                    *prev = Some(node.val);
                }
            } else {
                *prev = Some(node.val);
                *count = 1;
            }
            f(node.val, *count);
            inorder(&node.right, prev, count, f);
        }
    }
}
pub fn find_mode(root: TreeLink) -> Vec<i32> {
    let mut max = 0;
    let mut count = 0;
    let mut prev: Option<i32> = None;
    let mut modes: Vec<i32> = vec![];
    root.inorder(&mut prev, &mut count, &mut |_, count| {
        max = usize::max(count, max);
    });
    prev = None;
    count = 0;
    root.inorder(&mut prev, &mut count, &mut |val, count| {
        if count == max {
            modes.push(val);
        }
    });
    modes
}
/*
Minimum Absolute Difference in BST
Given a binary search tree with non-negative values, find the minimum absolute difference between values of any two nodes.

Example:

Input:

   1
    \
     3
    /
   2

Output:
1

Explanation:
The minimum absolute difference is 1, which is the difference between 2 and 1 (or between 2 and 3).

*/
pub trait Inorder {
    fn inorder(&self, prev: &mut Option<i32>, min: &mut i32);
}
impl Inorder for TreeLink {
    fn inorder(&self, prev: &mut Option<i32>, min: &mut i32) {
        if let Some(node) = self {
            let node = node.borrow();
            inorder(&node.left, prev, min);
            if let Some(prev_val) = prev.as_mut() {
                *min = (node.val - *prev_val).abs().min(*min);
                *prev_val = node.val;
            } else {
                *prev = Some(node.val);
            }
            inorder(&node.right, prev, min);
        }
    }
}
pub fn get_minimum_difference(root: TreeLink) -> i32 {
    let mut min = i32::MAX;
    let mut prev: Option<i32> = None;
    root.inorder(&mut prev, &mut min);
    min
}
/*
Diameter of Binary Tree
Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

Example:
Given a binary tree
          1
         / \
        2   3
       / \     
      4   5    
Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].
*/
pub trait Diameter {
    fn diameter(&self) -> i32;
    fn max_depth(&self, max: &mut i32) -> i32;
}
impl Diameter for TreeLink {
    fn diameter(&self) -> i32 {
        let mut max: i32 = 0;
        self.max_depth(&mut max);
        max
    }
    fn max_depth(&self, max: &mut i32) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let left = node.left.max_depth(max);
            let right = node.right.max_depth(max);
            *max = (*max).max(left + right);
            (left + 1).max(right + 1)
        } else {
            0
        }
    }
}
pub fn diameter_of_binary_tree(root: TreeLink) -> i32 {
    root.diameter()
}
/*
Binary Tree Tilt
Given the root of a binary tree, return the sum of every tree node's tilt.

The tilt of a tree node is the absolute difference between the sum of all left subtree node values and all right subtree node values. If a node does not have a left child, then the sum of the left subtree node values is treated as 0. The rule is similar if there the node does not have a right child.

 

Example 1:


Input: root = [1,2,3]
Output: 1
Explanation: 
Tilt of node 2 : |0-0| = 0 (no children)
Tilt of node 3 : |0-0| = 0 (no children)
Tile of node 1 : |2-3| = 1 (left subtree is just left child, so sum is 2; right subtree is just right child, so sum is 3)
Sum of every tilt : 0 + 0 + 1 = 1
*/
pub trait Tilt {
    fn find_tilt(&self, tilt: &mut i32) -> i32;
}
impl Tilt for TreeLink {
    fn find_tilt(&self, tilt: &mut i32) -> i32 {
        if let Some(node) = self {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            let left_sum = left.find_tilt(tilt);
            let right_sum = right.find_tilt(tilt);
            *tilt += (left_sum - right_sum).abs();
            node.val + left_sum + right_sum
        } else {
            0
        }
    }
}
pub fn find_tilt(root: TreeLink) -> i32 {
    let mut tilt = 0;
    root.find_tilt(&mut tilt);
    tilt
}
/*
Subtree of Another Tree
Given two non-empty binary trees s and t, check whether tree t has exactly the same structure and node values with a subtree of s. A subtree of s is a tree consists of a node in s and all of this node's descendants. The tree s could also be considered as a subtree of itself.

Example 1:
Given tree s:

     3
    / \
   4   5
  / \
 1   2
Given tree t:
   4 
  / \
 1   2
Return true, because t has the same structure and node values with a subtree of s.

*/
pub trait Subtree {
    fn is_subtree(&self, t: &TreeLink) -> bool;
}
impl Subtree for TreeLink {
    fn is_subtree(&self, t: &TreeLink) -> bool {
        if self == t { return true; }
        if let Some(node) = self {
            let left = &node.borrow().left;
            let right = &node.borrow().right;
            return left.is_subtree(t) || right.is_subtree(t);
        }
        false;
    }
}
pub fn is_subtree(s: TreeLink, t: TreeLink) -> bool {
    s.is_subtree(t)
}
/*
Construct String from Binary Tree
You need to construct a string consists of parenthesis and integers from a binary tree with the preorder traversing way.

The null node needs to be represented by empty parenthesis pair "()". And you need to omit all the empty parenthesis pairs that don't affect the one-to-one mapping relationship between the string and the original binary tree.

Example 1:
Input: Binary tree: [1,2,3,4]
       1
     /   \
    2     3
   /    
  4     

Output: "1(2(4))(3)"

Explanation: Originallay it needs to be "1(2(4)())(3()())", 
but you need to omit all the unnecessary empty parenthesis pairs. 
And it will be "1(2(4))(3)".
*/
pub trait Treetostring {
    fn tree_to_string(&self) -> String;
}
impl Treetostring for TreeLink {
    fn tree_to_string(&self) -> String {
        if let Some(node) = self {
            let mut node = node.borrow_mut();
            let left = node.left.take();
            let right = node.right.take();
            match (&left, &right) {
                (Some(_), Some(_)) => format!(
                "{}({})({})",
                node.val,
                left.tree_to_string(),
                right.tree_to_string(),
                ),
                (Some(_), None) => format!("{}({})", node.val, left.tree_to_string()),
                (None, Some(_)) => format!("{}()({})", node.val, right.tree_to_string()),
                (None, None) => format!("{}", node.val),
            }
        } else {
            "".to_string()
        }
    }
}
pub fn treetostring(root: TreeLink) -> String {
    root.tree_to_string()
}
/*
Merge Two Binary Trees
You are given two binary trees root1 and root2.

Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.

Return the merged tree.

Note: The merging process must start from the root nodes of both trees.

 

Example 1:


Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
Output: [3,4,5,5,4,null,7]
Example 2:

Input: root1 = [1], root2 = [1,2]
Output: [2,2]
*/
pub fn merge_trees(t1: TreeLink, t2: TreeLink) -> TreeLink {
    match (t1, t2) {
        (Some(n1), Some(n2)) => {
            let mut n1 = n1.borrow_mut();
            let mut n2 = n2.borrow_mut();
            tree!(
                n1.val + n2.val,
                merge_trees(n1.left.take(), n2.left.take()),
                merge_trees(n1.right.take(), n2.right.take())
            )
        }
        (None, Some(n2)) => Some(n2),
        (Some(n1), None) => Some(n1),
        (None, None) => None,
    }
}
////////////////
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
