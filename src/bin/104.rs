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
    let max_depth = 0;
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, mut max_depth: i32) -> i32 {
        if let Some(node) = node {
            max_depth += 1;
            let left = recurse(node.borrow().left.clone(), max_depth);
            let right = recurse(node.borrow().right.clone(), max_depth);
            max_depth = left.max(right);
        }
        max_depth
    }

    recurse(root, max_depth)
}

fn main() {}
