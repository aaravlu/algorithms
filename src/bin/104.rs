// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let left = recurse(node.left.clone());
            let right = recurse(node.right.clone());
            left.max(right) + 1
        } else {
            0
        }
    }

    recurse(root)
}

fn main() {}
