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
pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(node) = node {
            let mut node_borrow = node.borrow_mut();
            recurse(node_borrow.right.clone(), sum);
            *sum += node_borrow.val;
            node_borrow.val = *sum;
            recurse(node_borrow.left.clone(), sum);
        }
    }

    recurse(root.clone(), &mut 0);
    root
}

fn main() {}
