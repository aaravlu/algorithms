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

use core::cmp;
use std::cell::RefCell;
use std::rc::Rc;
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut diameter = 0;

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        if let Some(n) = node {
            let left = dfs(n.borrow().left.clone(), diameter);
            let right = dfs(n.borrow().right.clone(), diameter);
            *diameter = cmp::max(*diameter, left + right);
            cmp::max(left, right) + 1
        } else {
            0
        }
    }

    dfs(root, &mut diameter);
    diameter
}

fn main() {}
