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
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = node {
            let node_borrow = node.borrow();
            let (left_height, left_diameter) = recurse(node_borrow.left.clone());
            let (right_height, right_diameter) = recurse(node_borrow.right.clone());

            let height = left_height.max(right_height) + 1;
            let diameter = left_diameter
                .max(right_diameter)
                .max(left_height + right_height);
            (height, diameter)
        } else {
            (0, 0)
        }
    }

    recurse(root).1
}

fn main() {}
