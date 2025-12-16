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
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut diameter = 0;

    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        if let Some(node) = node {
            let node_borrow = node.borrow();
            let left = recurse(node_borrow.left.clone(), diameter);
            let right = recurse(node_borrow.right.clone(), diameter);
            *diameter = core::cmp::max(*diameter, left + right);
            core::cmp::max(left, right) + 1
        } else {
            0
        }
    }

    recurse(root, &mut diameter);
    diameter
}

fn main() {}
