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
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        if let Some(node) = node {
            let node_borrow = node.borrow();
            let node_val = node_borrow.val;

            let min_bool = min.is_none_or(|min| min < node_val);
            let max_bool = max.is_none_or(|max| max > node_val);

            min_bool
                && max_bool
                && recurse(node_borrow.left.clone(), min, Some(node_val))
                && recurse(node_borrow.right.clone(), Some(node_val), max)
        } else {
            true
        }
    }
    recurse(root, None, None)
}

fn main() {}
