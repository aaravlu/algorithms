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
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, max: Option<i32>, min: Option<i32>) -> bool {
        if let Some(node) = node {
            let node_borrow = node.borrow();
            let current_node_val = node_borrow.val;
            min.is_none_or(|min| min < current_node_val)
                && max.is_none_or(|max| max > current_node_val)
                && recurse(node_borrow.left.clone(), Some(current_node_val), min)
                && recurse(node_borrow.right.clone(), max, Some(current_node_val))
        } else {
            true
        }
    }

    recurse(root, None, None)
}

fn main() {}
